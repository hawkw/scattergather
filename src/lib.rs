use std::io::IoSlice;
use std::mem::{self, MaybeUninit};

pub struct IoSlices<'a> {
    bufs: &'a mut [MaybeUninit<IoSlice<'a>>],
    init: usize,
}

impl<'a> IoSlices<'a> {
    #[inline]
    pub fn new(bufs: &'a mut [IoSlice<'a>]) -> Self {
        let init = bufs.len();
        let bufs = unsafe { mem::transmute::<_, &mut [MaybeUninit<IoSlice<'_>>]>(bufs) };
        Self { bufs, init }
    }

    #[inline]
    pub fn uninit(bufs: &'a mut [MaybeUninit<IoSlice<'a>>]) -> Self {
        Self { bufs, init: 0 }
    }

    #[inline]
    pub fn push(&mut self, buf: IoSlice<'a>) -> bool {
        if let Some(b) = self.bufs.get_mut(self.init + 1) {
            *b = MaybeUninit::new(buf);
            self.init += 1;
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        std::cmp::min(self.bufs.len(), self.init)
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.bufs.len()
    }

    #[inline]
    pub fn bufs(&self) -> &[IoSlice<'a>] {
        let bufs = &self.bufs[..self.len()];
        unsafe { mem::transmute::<_, &[IoSlice<'_>]>(bufs) }
    }

    #[inline]
    pub fn bufs_mut(&mut self) -> &mut [IoSlice<'a>] {
        let len = self.len();
        let bufs = &mut self.bufs[..len];
        unsafe { mem::transmute::<_, &mut [IoSlice<'_>]>(bufs) }
    }
}

impl<'a> AsRef<[IoSlice<'a>]> for IoSlices<'a> {
    #[inline]
    fn as_ref(&self) -> &[IoSlice<'a>] {
        self.bufs()
    }
}

impl<'a> AsMut<[IoSlice<'a>]> for IoSlices<'a> {
    #[inline]
    fn as_mut(&mut self) -> &mut [IoSlice<'a>] {
        self.bufs_mut()
    }
}
