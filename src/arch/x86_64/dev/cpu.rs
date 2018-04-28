

#[inline]
pub unsafe hlt() {
    asm!("hlt");
}
