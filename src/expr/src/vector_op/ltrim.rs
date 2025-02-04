// Copyright 2022 Singularity Data
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use risingwave_common::array::{StringWriter, WrittenGuard};

use crate::Result;

/// Note: the behavior of `ltrim` in `PostgreSQL` and `trim_start` (or `trim_left`) in Rust
/// are actually different when the string is in right-to-left languages like Arabic or Hebrew.
/// Since we would like to simplify the implementation, currently we omit this case.
#[inline(always)]
pub fn ltrim(s: &str, writer: StringWriter<'_>) -> Result<WrittenGuard> {
    Ok(writer.write_ref(s.trim_start()))
}

#[cfg(test)]
mod tests {
    use risingwave_common::array::{Array, ArrayBuilder, Utf8ArrayBuilder};

    use super::*;

    #[test]
    fn test_ltrim() -> Result<()> {
        let cases = [
            (" \tHello\tworld\t", "Hello\tworld\t"),
            (" \t空I ❤️ databases空 ", "空I ❤️ databases空 "),
        ];

        for (s, expected) in cases {
            let mut builder = Utf8ArrayBuilder::new(1);
            let writer = builder.writer();
            let _guard = ltrim(s, writer)?;
            let array = builder.finish();
            let v = array.value_at(0).unwrap();
            assert_eq!(v, expected);
        }
        Ok(())
    }
}
