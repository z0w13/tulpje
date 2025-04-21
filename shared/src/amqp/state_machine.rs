pub(super) trait FromState<F>: Sized {
    fn from_state(state: Self) -> Self;
}
pub(super) trait IntoState<T>: Sized {
    fn into_state(state: T) -> T;
}

impl<F, T> IntoState<T> for F
where
    T: FromState<F>,
{
    fn into_state(state: T) -> T {
        T::from_state(state)
    }
}

#[macro_export]
macro_rules! state_transition {
    ($from:ident => $to:ident $(,)?) => {
        impl $crate::amqp::state_machine::FromState<$from> for $to {
            fn from_state(state: Self) -> Self {
                state
            }
        }
    };
    ($from:ident => [$($to:ident),*] $(,)?) => {
        $($crate::state_transition!($from => $to);)+
    };
}
