use crate::client::Minecraft;
use crate::settings::Component;
use lwjgl::opengl::Display;

// java.awt.Canvas of the deferred awt port, its size is the lwjgl window once the display exists
pub struct Canvas
{
    m_preferredWidth: i32,
    m_preferredHeight: i32,
}

impl Canvas
{
    // the size the packed frame gives the canvas before the window opens
    pub fn new(preferredWidth: i32, preferredHeight: i32) -> Canvas
    {
        Canvas { m_preferredWidth: preferredWidth, m_preferredHeight: preferredHeight }
    }
}

impl Component for Canvas
{
    fn getWidth(&self) -> i32
    {
        if Display::isCreated()
        {
            Display::getWindowSize().0
        }
        else
        {
            self.m_preferredWidth
        }
    }

    fn getHeight(&self) -> i32
    {
        if Display::isCreated()
        {
            Display::getWindowSize().1
        }
        else
        {
            self.m_preferredHeight
        }
    }
}

// java.awt.Frame of the deferred awt port, the lwjgl window and its title stand in for it
pub struct Frame;

impl Frame
{
    // the awt event thread stand-in, run once a frame where the original checks the display itself
    pub fn dispatchEvents(&self, minecraft: &Minecraft)
    {
        if Display::isCloseRequested()
        {
            self.windowClosing(minecraft);
        }
    }

    // the anonymous WindowAdapter; ending the run loop leads to the same System.exit(0) the join was waiting for
    fn windowClosing(&self, minecraft: &Minecraft) { minecraft.shutdown(); }
}
