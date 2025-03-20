use std::io::{self, Result};
use std::mem::MaybeUninit;

#[derive(Debug)]
/// Pre-allocated stack storage
/// can store up to `N` elements.
/// `N` is a const specified at compile time.
pub struct FlowerPot<T, const N: usize> {
    items: [MaybeUninit<T>; N],
    pos: usize,
}

impl<T, const N: usize> FlowerPot<T, N> {
    /// Creates a new `FlowerPot`
    /// with the `pos` field set to 0.
    pub fn new() -> FlowerPot<T, N> {
        let items = [const { MaybeUninit::uninit() }; N];

        Self { items, pos: 0 }
    }

    /// Returns `true` if `pos` is bigger than or equal to `N`
    /// else returns `false`.
    #[inline]
    pub const fn full(&self) -> bool {
        self.pos >= N
    }

    /// Returns `true` if `pos` is equal to 0.
    /// else returns false.
    #[inline]
    pub const fn empty(&self) -> bool {
        self.pos == 0
    }

    /// Returns the current amount of used space,
    /// the current implementation uses `checked_sub` on `pos`
    /// returning `0` on `None` and the value on `Some`.
    #[inline]
    pub fn len(&self) -> usize {
        match self.pos.checked_sub(1) {
            None => 0,
            Some(num) => num,
        }
    }

    /// Pushes an item to the `FlowerPot`.
    /// returns `Ok` if the operation was successful.
    /// if the container is full, returns `Err`
    pub fn push(&mut self, item: T) -> Result<()> {
        if self.full() {
            let err = io::Error::from(io::ErrorKind::StorageFull);

            return Err(err);
        }

        unsafe {
            let reference = &mut *(self.items.as_mut_ptr().add(self.pos));
            reference.write(item);

            self.pos += 1
        }

        Ok(())
    }

    /// Pops an item from the `FlowerPot`.
    /// returns `None` if the container is empty.
    pub fn pop(&mut self) -> Option<T> {
        if self.empty() {
            return None;
        }

        self.pos -= 1;

        let val = unsafe {
            let maybe = &*(self.items.as_mut_ptr().add(self.pos));
            maybe.assume_init_read()
        };

        Some(val)
    }

    /// Obtains an immutable reference to an item at an specified index.
    /// returns `None` if that index is out of bounds.
    pub fn get(&self, index: usize) -> Option<&T> {
        if index > self.pos {
            return None;
        }

        // SAFETY: The index we are passing is within the bounds.
        // Therefore it is safe to create an immutable reference.
        let reference = unsafe { &*(self.items.as_ptr().add(index) as *const T) };

        Some(reference)
    }

    /// Obtains a mutable reference to an item at an specified index.
    /// returns `None` if that index is out of bounds.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index > self.pos {
            return None;
        }

        // SAFETY: We possess exclusive access to the entire collection
        // and the index we are passing is within the bounds.
        // Therefore it is safe to create a mutable reference.
        let reference = unsafe { &mut *(self.items.as_ptr().add(index) as *mut T) };

        Some(reference)
    }

    /// Obtains an immutable reference to an item at an specified index.
    /// Does not check if the memory at the index is initialized.
    pub unsafe fn get_unchecked(&mut self, index: usize) -> &T {
        unsafe { &mut *(self.items.as_ptr().add(index) as *mut T) }
    }

    /// Obtains a mutable reference to an item at an specified index.
    /// Does not check if the memory at the index is initialized.
    pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        unsafe { &mut *(self.items.as_ptr().add(index) as *mut T) }
    }

    /// Obtains an immutable reference to the initialized part of the `FlowerPot`.
    /// if `pos` is `0` then returns a reference to an empty slice.
    pub fn get_init_slice(&self) -> &[T] {
        if self.pos == 0 {
            return &mut [];
        };

        let ptr = &self.items[0..self.pos];

        // SAFETY: `ptr` refers to a part of the slice ranging from the first element
        // at index `0` and the last at `self.pos`.
        // therefore we are creating a reference to a slice of initialized memory only.
        unsafe { &*(ptr as *const [MaybeUninit<T>] as *const [T]) }
    }

    /// Obtains a mutable reference to the initialized part of the `FlowerPot`.
    /// if `pos` is `0` then returns a reference to an empty slice.
    pub fn get_init_slice_mut(&mut self) -> &mut [T] {
        if self.pos == 0 {
            return &mut [];
        };

        let ptr = &mut self.items[0..self.pos];

        // SAFETY: `ptr` refers to a part of the slice ranging from the first element
        // at index `0` and the last at `self.pos`.
        // therefore we are creating a reference to a slice of initialized memory only.
        unsafe { &mut *(ptr as *mut [MaybeUninit<T>] as *mut [T]) }
    }
}

impl<T, const N: usize> std::ops::Drop for FlowerPot<T, N> {
    fn drop(&mut self) {
        if self.pos != 0 {
            let slice = &mut self.items[0..self.pos];

            for item in slice {
                // SAFETY: `item` originates from `slice`
                // `slice` is a slice of only initialized `MaybeUninit`s
                unsafe {
                    item.assume_init_drop();
                }
            }
        }
    }
}
