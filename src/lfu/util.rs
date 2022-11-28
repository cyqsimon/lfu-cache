use std::ptr::NonNull;

use crate::frequency_list::Node;

use super::Entry;

/// Removes the entry from the cache, cleaning up any values if necessary.
pub fn remove_entry_pointer<Key, Value>(mut node: Entry<Key, Value>, len: &mut usize) -> Value
where
{
    let owner = unsafe { node.owner.as_mut() };
    drop(Entry::detach_owned(NonNull::from(&mut node)));
    if owner.elements.is_none() {
        Node::detach(unsafe { *Box::from_raw(owner) });

        // Drop the node, since the node is empty now.
        // TODO: low frequency count optimization, where we don't dealloc
        // very low frequency counts since we're likely to just realloc them
        // sooner than later.
    }
    *len -= 1;

    node.value
}
