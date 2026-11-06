use crate::tileentity::{TILE_ENTITY, TileEntity, TileEntityClass};
use jvm::field::Field;
use jvm::string::JString;
use std::cell::Cell;
use std::rc::Rc;

pub struct SignExt
{
    pub m_signText: Field<[Option<JString>; 4]>,
    pub m_lineBeingEdited: Cell<i32>,
    m_editable: Cell<bool>,
}

pub fn fields(this: &TileEntity) -> &SignExt { this.ext::<SignExt>() }

pub const CLASS: TileEntityClass = TileEntityClass {
    m_name: "TileEntitySign",
    m_parent: Some(&TILE_ENTITY),
    writeToNBT: |this, nbt| {
        (TILE_ENTITY.writeToNBT)(this, nbt);
        let text: [Option<JString>; 4] = fields(this).m_signText.get();
        nbt.setString("Text1", text[0].clone());
        nbt.setString("Text2", text[1].clone());
        nbt.setString("Text3", text[2].clone());
        nbt.setString("Text4", text[3].clone());
    },
    readFromNBT: |this, nbt| {
        let sign: &SignExt = fields(this);
        sign.m_editable.set(false);
        (TILE_ENTITY.readFromNBT)(this, nbt);
        for i in 0..4
        {
            let mut line: Option<JString> = nbt.getString(&format!("Text{}", i + 1));
            if let Some(text) = &line
                && text.length() > 15
            {
                line = Some(text.substring(0, 15));
            }
            sign.m_signText.update(|lines| lines[i] = line);
        }
    },
    ..TILE_ENTITY
};

pub fn create() -> Rc<TileEntity>
{
    let blank: Option<JString> = Some(JString::new());
    let ext: SignExt = SignExt {
        m_signText: Field::new([blank.clone(), blank.clone(), blank.clone(), blank]),
        m_lineBeingEdited: Cell::new(-1),
        m_editable: Cell::new(true),
    };
    TileEntity::new(&CLASS, Box::new(ext))
}
