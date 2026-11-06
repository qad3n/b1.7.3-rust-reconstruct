use lwjgl::buffers::{ByteBuffer, ByteOrder, FloatBuffer, IntBuffer};
use lwjgl::opengl::GL11;
use std::cell::RefCell;

thread_local! {
    static DISPLAY_LISTS: RefCell<Vec<i32>> = const { RefCell::new(Vec::new()) };
    static TEXTURE_NAMES: RefCell<Vec<i32>> = const { RefCell::new(Vec::new()) };
}

pub struct GLAllocation;

impl GLAllocation
{
    pub fn generateDisplayLists(s: i32) -> i32
    {
        let n: i32 = GL11::glGenLists(s);
        DISPLAY_LISTS.with(|lists| lists.borrow_mut().extend([n, s]));
        n
    }

    pub fn generateTextureNames(lists: &IntBuffer)
    {
        GL11::glGenTextures(lists);
        TEXTURE_NAMES.with(|names| {
            let mut names = names.borrow_mut();
            for i in lists.position()..lists.limit()
            {
                names.push(lists.getAt(i));
            }
        });
    }

    pub fn releaseList(list: i32)
    {
        DISPLAY_LISTS.with(|lists| {
            let mut lists = lists.borrow_mut();
            let Some(n) = lists.iter().position(|&entry| entry == list)
            else
            {
                jvm::throwable::throw(&jvm::throwable::ARRAY_INDEX, "-1");
            };
            GL11::glDeleteLists(lists[n], lists[n + 1]);
            lists.drain(n..n + 2);
        });
    }

    pub fn deleteTexturesAndDisplayLists()
    {
        let displayLists: Vec<i32> = DISPLAY_LISTS.with(|lists| lists.borrow().clone());
        for pair in displayLists.chunks(2)
        {
            GL11::glDeleteLists(pair[0], pair[1]);
        }
        let textureNames: Vec<i32> = TEXTURE_NAMES.with(|names| names.borrow().clone());
        let intBuffer: IntBuffer = Self::createDirectIntBuffer(textureNames.len() as i32);
        intBuffer.flip();
        GL11::glDeleteTextures(&intBuffer);
        for name in &textureNames
        {
            intBuffer.put(*name);
        }
        intBuffer.flip();
        GL11::glDeleteTextures(&intBuffer);
        DISPLAY_LISTS.with(|lists| lists.borrow_mut().clear());
        TEXTURE_NAMES.with(|names| names.borrow_mut().clear());
    }

    pub fn createDirectByteBuffer(capacity: i32) -> ByteBuffer
    {
        ByteBuffer::allocateDirect(capacity).order(ByteOrder::nativeOrder())
    }

    pub fn createDirectIntBuffer(capacity: i32) -> IntBuffer
    {
        Self::createDirectByteBuffer(capacity << 2).asIntBuffer()
    }

    pub fn createDirectFloatBuffer(capacity: i32) -> FloatBuffer
    {
        Self::createDirectByteBuffer(capacity << 2).asFloatBuffer()
    }
}
