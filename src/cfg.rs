#[test]
fn test() {
    use crate::*;
    use std::borrow::Cow;

    let headers: HashMap<_, _, BuildHasherDefault<XxHash3_64>> =
        HashMap::with_hasher(BuildHasherDefault::default());
    let data: Vec<u8> = vec![];
    let body: Cow<'_, Vec<u8>> = Compress::from(&headers).decode(&data, 1_024_000);
    assert_eq!(*body, data);

    let _ = Compress::Gzip.encode(&[], 1_024_000);
    let _ = Compress::Deflate.encode(&[], 1_024_000);
    let _ = Compress::Br.encode(&[], 1_024_000);

    let _ = Compress::Gzip.decode(&[], 1_024_000);
    let _ = Compress::Deflate.decode(&[], 1_024_000);
    let _ = Compress::Br.decode(&[], 1_024_000);
}
