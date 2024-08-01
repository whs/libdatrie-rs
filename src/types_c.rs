/// cbindgen:ignore
#[derive(Eq, PartialEq, Copy, Clone)]
#[repr(transparent)]
pub struct Bool(u32);

pub(crate) const DA_TRUE: Bool = Bool(1);
pub(crate) const DA_FALSE: Bool = Bool(0);
pub(crate) const FALSE: Bool = DA_FALSE;
pub(crate) const TRUE: Bool = DA_TRUE;

impl From<bool> for Bool {
    fn from(value: bool) -> Self {
        match value {
            true => TRUE,
            false => FALSE,
        }
    }
}

impl Into<bool> for Bool {
    fn into(self) -> bool {
        self.0 == 1
    }
}
