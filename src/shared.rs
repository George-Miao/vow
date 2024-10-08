macro_rules! shared_impl {
    ($Vow:ident<$T:ident, $F:ident: $bound:ident> $(, $async:tt + $await:tt)?) => {
        const _: () = {
            use crate::Data;

            const NO_VALUE: &str =
                "Value was taken and not returned, likely due to an error during async io";

            impl<$T, $F> $Vow<$T, $F> {
                /// Get the value.
                pub fn get(&self) -> &T {
                    &self.value.as_ref().expect(NO_VALUE)
                }

                pub(crate) fn take(&mut self) -> T {
                    std::mem::take(&mut self.value).expect(NO_VALUE)
                }
            }

            impl<$T, $F> $Vow<$T, $F> where
                T: Data,
                F: $bound
            {
                /// Set the value.
                pub $($async)? fn set(&mut self, value: T) -> VowResult<()> {
                    self.value = Some(self.io.sync(Some(value), true)$(.$await)??);
                    Ok(())
                }

                /// Map the value.
                pub $($async)? fn map<U>(&mut self, f: U) -> VowResult<()>
                where
                    U: FnOnce(T) -> T + MaybeSend,
                {
                    let val = f(self.take());
                    self.value = Some(self.io.sync(Some(val), true)$(.$await)??);
                    Ok(())
                }

                /// Update the value.
                pub $($async)? fn update<U>(&mut self, f: U) -> VowResult<()>
                where
                    U: FnOnce(&mut T) + MaybeSend,
                {
                    let mut val = self.take();
                    f(&mut val);
                    self.value = Some(self.io.sync(Some(val), true)$(.$await)??);
                    Ok(())
                }

                /// Force reload the value.
                pub $($async)? fn force_reload(&mut self) -> VowResult<()> {
                    self.value = Some(self.io.sync(None, false)$(.$await)??);
                    Ok(())
                }


                /// Flush the content down to disk.
                pub $($async)? fn flush(&mut self) -> VowResult<()> {
                    self.io.file.flush()$(.$await)??;
                    Ok(())
                }
            }

            impl<$T, $F> ::std::ops::Deref for $Vow<$T, $F> {
                type Target = T;

                fn deref(&self) -> &Self::Target {
                    self.get()
                }
            }
        };
    };
}

pub(crate) use shared_impl;
