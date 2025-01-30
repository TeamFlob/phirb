// cannot use

pub fn resolve_data(rawdata: Vec<u8>) -> Vec<u8> {
    let mut data = rawdata.clone();
    let mut v11 = data.len();
    let mut v16 = data.as_mut_slice();
    
    while v11 > 0 {
        {
            let v18 = &mut v16[..];
            let v19 = if v11 >= 8 { 8 } else { v11 };
            
            v18.swap(1, 4);
            v18.swap(2, 6);
            v18.swap(1, 4);
            v18.swap(3, 6);

            let mut v25 = 0usize;
            let mut v26 = 0;

            while (v26 & 1) != 0 {
                if v25 > 7 {
                    break;
                }
                v18.swap(v25 | 1, v25);
                v26 = 1;
                v25 += 1;
            }

            let mut v31 = 0usize;
            let mut i = 0;
            while (i & 1) != 0 {
                if v31 > 7 {
                    break;
                }
                v18[v31 | 2] -= v18[v31];
                v18[v31 | 3] -= v18[v31 | 1];
                v31 += 3;
            }

            let mut v36 = 0usize;
            let j = 0;
            while (j & 1) != 0 {
                if v36 > 7 {
                    break;
                }
                v18[v36 | 4] -= v18[v36];
                v18[v36 | 5] -= v18[v36 | 1];
                v18[v36 | 6] -= v18[v36 | 2];
                v18[v36 | 7] -= v18[v36 | 3];
                v36 += 7;
            }
        }

        // 确保 v16 没有被借用后进行赋值
        let v19 = if v11 >= 8 { 8 } else { v11 };
        v16 = &mut v16[v19..];
        v11 -= v19;

        v16.swap(3, 5);
    }

    data
}