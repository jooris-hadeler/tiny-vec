use std::{fmt::Debug, hash::Hash, mem, vec::IntoIter};

#[cfg(test)]
mod tests;

/// A vector implementation that can store up to `STACK_CAPACITY` elements
/// on the stack before moving its elements to the heap.
#[derive(Clone)]
pub struct TinyVec<T: Sized, const STACK_CAPACITY: usize> {
    inner: TinyVecInner<T, STACK_CAPACITY>,
    length: usize,
}

#[derive(Clone)]
enum TinyVecInner<T: Sized, const STACK_CAPACITY: usize> {
    Stack([Option<T>; STACK_CAPACITY]),
    Heap(Vec<T>),
}

impl<T: Sized, const STACK_CAPACITY: usize> TinyVec<T, STACK_CAPACITY> {
    /// Creates a new empty [`TinyVec`].
    pub fn new() -> Self {
        Self {
            inner: TinyVecInner::Stack([const { None }; STACK_CAPACITY]),
            length: 0,
        }
    }

    /// This method makes sure that we spill onto the heap, if the `STACK_CAPACITY` is reached.
    /// This makes sure that we never overflow the stack buffer.
    fn spill(&mut self) {
        // return early if we are on the heap or there are fewer items than the STACK_CAPACITY allows
        if self.length < STACK_CAPACITY || matches!(self.inner, TinyVecInner::Heap(..)) {
            return;
        }

        // replace the stack array with a vector on the heap
        let TinyVecInner::Stack(array) =
            mem::replace(&mut self.inner, TinyVecInner::Heap(Vec::new()))
        else {
            // NOTE: we will never spill unless we are currently allocated on the heap,
            //       therefore we can safely assume this case is impossible.
            unreachable!();
        };

        let TinyVecInner::Heap(heap) = &mut self.inner else {
            // NOTE: we just spilled onto the stack `inner` cannot be of variant `Stack`
            //       therefore we can safely assume this case is impossible.
            unreachable!();
        };

        // move all items from the stack to the heap
        heap.extend(array.into_iter().map(|elm| elm.unwrap()))
    }

    /// Pushes an element onto the [`TinyVec`] if we have reached the `STACK_CAPACITY` we spill onto the heap.
    pub fn push(&mut self, item: T) {
        self.spill();

        match &mut self.inner {
            TinyVecInner::Stack(stack) => stack[self.length] = Some(item),
            TinyVecInner::Heap(heap) => heap.push(item),
        }

        self.length += 1;
    }

    /// Pops an element of of the [`TinyVec`], this however does not revert spillage.
    pub fn pop(&mut self) -> Option<T> {
        if self.length == 0 {
            return None;
        }

        self.length -= 1;

        match &mut self.inner {
            TinyVecInner::Stack(stack) => stack[self.length].take(),
            TinyVecInner::Heap(heap) => heap.pop(),
        }
    }

    /// Gets the element at a given index if it exists.
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.length {
            return None;
        }

        match &self.inner {
            TinyVecInner::Stack(stack) => stack[index].as_ref(),
            TinyVecInner::Heap(heap) => heap.get(index),
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.length {
            return None;
        }

        match &mut self.inner {
            TinyVecInner::Stack(stack) => stack[index].as_mut(),
            TinyVecInner::Heap(heap) => heap.get_mut(index),
        }
    }

    /// Returns whether or not the [`TinyVec`] has spilled onto the heap.
    pub fn has_spilled(&self) -> bool {
        matches!(self.inner, TinyVecInner::Heap(..))
    }

    /// Returns `true` if the [`TinyVec`] contains no elements, otherwise `false`.
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Returns the length of the [`TinyVec`].
    pub fn len(&self) -> usize {
        self.length
    }

    /// Returns an [`Iterator`] over the items of the [`TinyVec`].
    pub fn iter(&self) -> TinyVecIter<'_, T, STACK_CAPACITY> {
        TinyVecIter { vec: self, idx: 0 }
    }

    /// Extends the [`TinyVec`] by the elements of a given [`Iterator`].
    pub fn extend<I: Iterator<Item = T>>(&mut self, iter: I) {
        for elm in iter {
            self.push(elm);
        }
    }
}

impl<T: Sized, const N: usize> Default for TinyVec<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Sized + Debug, const N: usize> Debug for TinyVec<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T: Sized + PartialEq, const A: usize, const B: usize> PartialEq<TinyVec<T, A>>
    for TinyVec<T, B>
{
    fn eq(&self, other: &TinyVec<T, A>) -> bool {
        // check lengths
        if self.length != other.length {
            return false;
        }

        // check each element for equality
        for (a, b) in self.iter().zip(other.iter()) {
            if a.ne(b) {
                return false;
            }
        }

        // both vecs are equal at this point
        true
    }
}

impl<T: Sized + Eq, const N: usize> Eq for TinyVec<T, N> {}

impl<T: Sized, const N: usize, I: Iterator<Item = T>> From<I> for TinyVec<T, N> {
    fn from(value: I) -> Self {
        let mut tv = Self::new();
        tv.extend(value);
        tv
    }
}

impl<T: Sized + Hash, const N: usize> Hash for TinyVec<T, N> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for elm in self.iter().enumerate() {
            elm.hash(state);
        }
    }
}

impl<T: Sized, const N: usize> IntoIterator for TinyVec<T, N> {
    type Item = T;
    type IntoIter = TinyVecIntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        TinyVecIntoIter {
            inner: match self.inner {
                TinyVecInner::Stack(stack) => TinyVecIntoIterInner::Stack(stack),
                TinyVecInner::Heap(heap) => TinyVecIntoIterInner::Heap(heap.into_iter()),
            },
            idx: 0,
        }
    }
}

pub struct TinyVecIntoIter<T: Sized, const STACK_CAPACITY: usize> {
    inner: TinyVecIntoIterInner<T, STACK_CAPACITY>,
    idx: usize,
}

enum TinyVecIntoIterInner<T: Sized, const STACK_CAPACITY: usize> {
    Stack([Option<T>; STACK_CAPACITY]),
    Heap(IntoIter<T>),
}

impl<T: Sized, const STACK_CAPACITY: usize> Iterator for TinyVecIntoIter<T, STACK_CAPACITY> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.idx;
        self.idx += 1;

        match &mut self.inner {
            TinyVecIntoIterInner::Stack(stack) => {
                if idx >= stack.len() {
                    return None;
                }

                stack[idx].take()
            }
            TinyVecIntoIterInner::Heap(heap) => heap.next(),
        }
    }
}

pub struct TinyVecIter<'a, T: Sized, const STACK_CAPACITY: usize> {
    vec: &'a TinyVec<T, STACK_CAPACITY>,
    idx: usize,
}

impl<'a, T: Sized, const STACK_CAPACITY: usize> Iterator for TinyVecIter<'a, T, STACK_CAPACITY> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        // get the current index and increment
        let idx = self.idx;
        self.idx += 1;

        // get the element from the vec
        match &self.vec.inner {
            TinyVecInner::Stack(stack) => {
                // return `None` if we are out of bounds
                if self.idx >= self.vec.length {
                    return None;
                }

                stack[idx].as_ref()
            }
            TinyVecInner::Heap(heap) => heap.get(idx),
        }
    }
}
