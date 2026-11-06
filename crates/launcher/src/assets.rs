use serde_json::Value;
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};

const CLIENT_JAR_ENV: &str = "B173_CLIENT_JAR";
const SOUNDS_DIR_ENV: &str = "B173_SOUNDS_DIR";
const LAUNCHER_JAR: &str = ".minecraft/versions/b1.7.3/b1.7.3.jar";
const VERSION_MANIFEST_URL: &str = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";
const VERSION_ID: &str = "b1.7.3";
const CLIENT_JAR_SHA1: &str = "43db9b498cb67058d2e12d394e6507722e71bb45";
const CLIENT_JAR_SIZE: u64 = 1465375;
const RESOURCES_URL: &str = "https://resources.download.minecraft.net";
const MCASSET_URL: &str = "https://mcasset.cloud/b1.7.3/assets";

const CLIENT_STAMP: &str = ".client-jar.sha1";
const SOUNDS_STAMP: &str = ".sounds-installed";

const SOUND_PREFIXES: &[&str] = &["sound/", "newsound/", "music/", "newmusic/", "streaming/"];

#[derive(Debug, thiserror::Error)]
pub enum FetchError
{
    #[error("i/o on {path}")]
    Io
    {
        path: PathBuf, source: std::io::Error
    },
    #[error("download of {url} failed: {message}")]
    Download
    {
        url: String, message: String
    },
    #[error("unexpected metadata from {url}: {message}")]
    Metadata
    {
        url: String, message: String
    },
    #[error("sha1 mismatch for {what}: expected {expected}, got {actual}")]
    Sha1
    {
        what: String, expected: String, actual: String
    },
    #[error("size mismatch for {what}: expected {expected}, got {actual}")]
    Size
    {
        what: String, expected: u64, actual: u64
    },
    #[error("client jar {path}: {message}")]
    Zip
    {
        path: String, message: String
    },
}

struct Remote
{
    m_url: String,
    m_sha1: String,
    m_size: u64,
}

fn clientInstalled(root: &Path) -> bool
{
    std::fs::read_to_string(root.join(CLIENT_STAMP)).is_ok_and(|stamp| stamp.trim() == CLIENT_JAR_SHA1)
}

fn soundsInstalled(root: &Path, resources: &Path) -> bool
{
    std::fs::read_to_string(root.join(SOUNDS_STAMP)).is_ok_and(|stamp| Path::new(&stamp) == resources)
}

pub fn install(root: &Path, resources: &Path, clientJar: Option<PathBuf>) -> Result<(), FetchError>
{
    if !clientInstalled(root)
    {
        fetchClient(root, clientJar)?;
    }
    if !soundsInstalled(root, resources)
    {
        match localSounds()
        {
            Some(folder) => copySounds(resources, &folder)?,
            None => fetchSounds(resources)?,
        }
        write(&root.join(SOUNDS_STAMP), resources.to_string_lossy().as_bytes())?;
    }
    Ok(())
}

fn localSounds() -> Option<PathBuf>
{
    let folder: PathBuf = PathBuf::from(std::env::var_os(SOUNDS_DIR_ENV)?);
    folder.is_dir().then_some(folder)
}

fn copySounds(resources: &Path, source: &Path) -> Result<(), FetchError>
{
    eprintln!("beta173_launcher: copying sounds from {} into {}", source.display(), resources.display());
    for prefix in SOUND_PREFIXES
    {
        let folder: PathBuf = source.join(prefix.trim_end_matches('/'));
        if folder.is_dir()
        {
            copyTree(&folder, &resources.join(prefix.trim_end_matches('/')))?;
        }
    }
    Ok(())
}

fn copyTree(from: &Path, to: &Path) -> Result<(), FetchError>
{
    let entries: std::fs::ReadDir =
        std::fs::read_dir(from).map_err(|source| FetchError::Io { path: from.to_path_buf(), source })?;
    for entry in entries
    {
        let entry: std::fs::DirEntry = entry.map_err(|source| FetchError::Io { path: from.to_path_buf(), source })?;
        let path: PathBuf = entry.path();
        let destination: PathBuf = to.join(entry.file_name());
        if path.is_dir()
        {
            copyTree(&path, &destination)?;
        }
        else
        {
            let bytes: Vec<u8> =
                std::fs::read(&path).map_err(|source| FetchError::Io { path: path.clone(), source })?;
            write(&destination, &bytes)?;
        }
    }
    Ok(())
}

fn localJar(clientJar: Option<PathBuf>) -> Option<PathBuf>
{
    if let Some(path) = clientJar.or_else(|| std::env::var_os(CLIENT_JAR_ENV).map(PathBuf::from))
    {
        return Some(path);
    }
    let launcherJar: PathBuf = PathBuf::from(std::env::var_os("HOME")?).join(LAUNCHER_JAR);
    launcherJar.is_file().then_some(launcherJar)
}

fn fetchClient(root: &Path, clientJar: Option<PathBuf>) -> Result<(), FetchError>
{
    let (name, bytes): (String, Vec<u8>) = match localJar(clientJar)
    {
        Some(path) =>
        {
            eprintln!("beta173_launcher: extracting assets from {}", path.display());
            let bytes: Vec<u8> =
                std::fs::read(&path).map_err(|source| FetchError::Io { path: path.clone(), source })?;
            (path.display().to_string(), bytes)
        }
        None =>
        {
            eprintln!("beta173_launcher: no local client jar, downloading the {VERSION_ID} client from Mojang");
            match downloadClientJar()
            {
                Ok(bytes) => (format!("{VERSION_ID} client jar"), bytes),
                Err(error) =>
                {
                    eprintln!(
                        "beta173_launcher: copy the files of {MCASSET_URL} into {} to install the assets by hand",
                        root.display()
                    );
                    return Err(error);
                }
            }
        }
    };
    verify(&name, &bytes, CLIENT_JAR_SHA1, CLIENT_JAR_SIZE)?;
    extractJar(root, &name, &bytes)?;
    write(&root.join(CLIENT_STAMP), CLIENT_JAR_SHA1.as_bytes())
}

fn downloadClientJar() -> Result<Vec<u8>, FetchError>
{
    let version: Value = versionJson()?;
    let client: Remote = remote(&version, "/downloads/client", "url")?;
    if client.m_sha1 != CLIENT_JAR_SHA1
    {
        return Err(FetchError::Sha1 {
            what: "the version json client entry".to_owned(),
            expected: CLIENT_JAR_SHA1.to_owned(),
            actual: client.m_sha1,
        });
    }
    download(&client.m_url)
}

fn fetchSounds(resources: &Path) -> Result<(), FetchError>
{
    let version: Value = versionJson()?;
    let indexEntry: Remote = remote(&version, "/assetIndex", "url")?;
    let indexBytes: Vec<u8> = download(&indexEntry.m_url)?;
    verify(&indexEntry.m_url, &indexBytes, &indexEntry.m_sha1, indexEntry.m_size)?;
    let index: Value = parseJson(&indexEntry.m_url, &indexBytes)?;
    let objects: &serde_json::Map<String, Value> = index
        .get("objects")
        .and_then(Value::as_object)
        .ok_or_else(|| FetchError::Metadata { url: indexEntry.m_url.clone(), message: "no objects map".to_owned() })?;
    let wanted: Vec<(&String, &Value)> =
        objects.iter().filter(|(name, _)| SOUND_PREFIXES.iter().any(|prefix| name.starts_with(prefix))).collect();
    eprintln!("beta173_launcher: downloading {} sound files into {}", wanted.len(), resources.display());
    for (name, object) in wanted
    {
        let hash: String = stringField(&indexEntry.m_url, object, "hash")?;
        let size: u64 = object.get("size").and_then(Value::as_u64).unwrap_or(0);
        let folder: &str = hash.get(..2).ok_or_else(|| FetchError::Metadata {
            url: indexEntry.m_url.clone(),
            message: format!("short hash for {name}"),
        })?;
        let destination: PathBuf = resources.join(name);
        if std::fs::read(&destination).is_ok_and(|bytes| verify(name, &bytes, &hash, size).is_ok())
        {
            continue;
        }
        let url: String = format!("{RESOURCES_URL}/{folder}/{hash}");
        let bytes: Vec<u8> = download(&url)?;
        verify(name, &bytes, &hash, size)?;
        write(&destination, &bytes)?;
    }
    Ok(())
}

fn versionJson() -> Result<Value, FetchError>
{
    let manifest: Value = parseJson(VERSION_MANIFEST_URL, &download(VERSION_MANIFEST_URL)?)?;
    let entry: &Value = manifest
        .get("versions")
        .and_then(Value::as_array)
        .and_then(|versions| {
            versions.iter().find(|version| version.get("id").and_then(Value::as_str) == Some(VERSION_ID))
        })
        .ok_or_else(|| FetchError::Metadata {
            url: VERSION_MANIFEST_URL.to_owned(),
            message: format!("no {VERSION_ID} entry"),
        })?;
    let url: String = stringField(VERSION_MANIFEST_URL, entry, "url")?;
    let bytes: Vec<u8> = download(&url)?;
    let sha1: String = stringField(VERSION_MANIFEST_URL, entry, "sha1")?;
    verify(&url, &bytes, &sha1, 0)?;
    parseJson(&url, &bytes)
}

fn remote(json: &Value, pointer: &str, urlKey: &str) -> Result<Remote, FetchError>
{
    let entry: &Value = json
        .pointer(pointer)
        .ok_or_else(|| FetchError::Metadata { url: VERSION_ID.to_owned(), message: format!("missing {pointer}") })?;
    Ok(Remote {
        m_url: stringField(VERSION_ID, entry, urlKey)?,
        m_sha1: stringField(VERSION_ID, entry, "sha1")?,
        m_size: entry.get("size").and_then(Value::as_u64).unwrap_or(0),
    })
}

fn stringField(source: &str, json: &Value, key: &str) -> Result<String, FetchError>
{
    json.get(key)
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| FetchError::Metadata { url: source.to_owned(), message: format!("missing {key}") })
}

fn parseJson(url: &str, bytes: &[u8]) -> Result<Value, FetchError>
{
    serde_json::from_slice(bytes)
        .map_err(|error| FetchError::Metadata { url: url.to_owned(), message: error.to_string() })
}

fn download(url: &str) -> Result<Vec<u8>, FetchError>
{
    let failed = |message: String| FetchError::Download { url: url.to_owned(), message };
    let mut response: ureq::http::Response<ureq::Body> =
        ureq::get(url).call().map_err(|error| failed(error.to_string()))?;
    response.body_mut().with_config().limit(u64::MAX).read_to_vec().map_err(|error| failed(error.to_string()))
}

fn verify(what: &str, bytes: &[u8], sha1: &str, size: u64) -> Result<(), FetchError>
{
    if size != 0 && bytes.len() as u64 != size
    {
        return Err(FetchError::Size { what: what.to_owned(), expected: size, actual: bytes.len() as u64 });
    }
    let actual: String = sha1_smol::Sha1::from(bytes).digest().to_string();
    if !actual.eq_ignore_ascii_case(sha1)
    {
        return Err(FetchError::Sha1 { what: what.to_owned(), expected: sha1.to_owned(), actual });
    }
    Ok(())
}

fn extractJar(root: &Path, name: &str, bytes: &[u8]) -> Result<(), FetchError>
{
    let zipError = |error: zip::result::ZipError| FetchError::Zip { path: name.to_owned(), message: error.to_string() };
    let mut archive: zip::ZipArchive<Cursor<&[u8]>> = zip::ZipArchive::new(Cursor::new(bytes)).map_err(zipError)?;
    for index in 0..archive.len()
    {
        let mut entry: zip::read::ZipFile<'_, Cursor<&[u8]>> = archive.by_index(index).map_err(zipError)?;
        let Some(relative) = entry.enclosed_name()
        else
        {
            continue;
        };
        if entry.is_dir() || relative.starts_with("META-INF") || relative.extension().is_some_and(|ext| ext == "class")
        {
            continue;
        }
        let mut contents: Vec<u8> = Vec::new();
        entry.read_to_end(&mut contents).map_err(|source| FetchError::Io { path: relative.clone(), source })?;
        write(&root.join(&relative), &contents)?;
    }
    eprintln!("beta173_launcher: assets extracted into {}", root.display());
    Ok(())
}

fn write(path: &Path, bytes: &[u8]) -> Result<(), FetchError>
{
    if let Some(parent) = path.parent()
    {
        std::fs::create_dir_all(parent).map_err(|source| FetchError::Io { path: parent.to_owned(), source })?;
    }
    std::fs::write(path, bytes).map_err(|source| FetchError::Io { path: path.to_owned(), source })
}
