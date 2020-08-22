mod threshold_enc;
mod threshold_sig;

fn main() {
    crate::threshold_enc::threshold_encode();
    crate::threshold_sig::threshold_signature();
}