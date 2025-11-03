use std::{any::type_name_of_val, cmp::Ordering, fmt, ops::Deref, rc::Rc};

/// # A handle to a topological object
///
/// What a handle provides over just owning the object, is to provide the object
/// with an identity that is distinct from any other objects, even between
/// objects that are equal.
///
/// This was important in the old architecture, where geometry was only defined
/// locally. With local geometry (a point being defined as a one-dimensional
/// coordinate on a curve, for example), multiple points that are supposed to be
/// coincident might not actually come out the same when converted to 3D.
///
/// In that case, knowing the identity of a vertex allows you to only convert
/// only one of the local representations, then reuse the result for all others,
/// avoiding any problems from numerical inaccuracy.
///
/// However, in this experiment, all geometry is stored in 3D and converted back
/// to local representations as necessary, identity irrelevant in that case.
///
/// Maybe knowing the identity of a topological object provides some other
/// advantage that I'm not aware of right now. If not, then this could possibly
/// be removed.
pub struct Handle<T> {
    inner: Rc<T>,
}

impl<T> Handle<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: Rc::new(inner),
        }
    }
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> Deref for Handle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> Eq for Handle<T> {}

impl<T> Ord for Handle<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        let [self_ptr, other_ptr] =
            [self, other].map(|handle| Rc::as_ptr(&handle.inner));

        self_ptr.cmp(&other_ptr)
    }
}

impl<T> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.inner, &other.inner)
    }
}

impl<T> PartialOrd for Handle<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> fmt::Debug for Handle<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let type_name = short_type_name_of_val(self);
        let address = Rc::as_ptr(&self.inner);
        let object = &self.inner;

        write!(f, "{type_name}: {address:?} => ")?;

        if f.alternate() {
            write!(f, "{object:#?}")?;
        } else {
            write!(f, "{object:?}")?;
        }

        Ok(())
    }
}

fn short_type_name_of_val<T>(val: &T) -> String {
    let full_name = type_name_of_val(val);

    let [type_parameters_open, type_parameters_close] =
        [full_name.find("<"), full_name.rfind(">")].map(|maybe_pos| {
            let Some(pos) = maybe_pos else {
                unreachable!(
                    "Only using this function for `Handle`, which has a type \
                    parameter."
                );
            };

            pos
        });

    let raw_name = shorten_type_name(&full_name[..type_parameters_open]);
    let type_parameter = shorten_type_name(
        &full_name[type_parameters_open + 1..type_parameters_close],
    );

    format!("{raw_name}<{type_parameter}>")
}

fn shorten_type_name(name: &str) -> &str {
    name.rsplit_once("::")
        .map(|(_, short)| short)
        .unwrap_or(name)
}
