use std::{any::type_name_of_val, cmp::Ordering, fmt, ops::Deref, rc::Rc};

/// # A handle to a topological object
///
/// Topological objects form a graph. There may exist multiple references to a
/// given object, none of which is privileged over the others in any way. There
/// is no clear owner.
///
/// Referring to these objects via a handle allows them to be shared. On top of
/// that, `Handle` gives a clear identity to such an object. This provides a
/// cheap and unambiguous way to distinguish it from other objects, whether
/// those are equal or not.
///
/// Having such an unambiguous identity is useful in multiple ways:
///
/// - It can be used to express intent, for example that a shared vertex is
///   actually meant to be one object, as opposed to multiple that happen to be
///   coincident.
/// - It makes it straight-forward to associate other data with an object, for
///   example a cached approximation that can later be reused.
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
