use crate::*;

pub use decoder::DataBarDecoder;
pub use encoder::DataBarEncoder;

pub const SBE_BLOCK_LENGTH: u16 = 50;
pub const SBE_TEMPLATE_ID: u16 = 204;
pub const SBE_SCHEMA_ID: u16 = 1;
pub const SBE_SCHEMA_VERSION: u16 = 1;
pub const SBE_SEMANTIC_VERSION: &str = "5.2";

pub mod encoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct DataBarEncoder<'a> {
        buf: WriteBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
    }

    impl<'a> Writer<'a> for DataBarEncoder<'a> {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            &mut self.buf
        }
    }

    impl<'a> Encoder<'a> for DataBarEncoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> DataBarEncoder<'a> {
        pub fn wrap(mut self, buf: WriteBuf<'a>, offset: usize) -> Self {
            let limit = offset + SBE_BLOCK_LENGTH as usize;
            self.buf = buf;
            self.initial_offset = offset;
            self.offset = offset;
            self.limit = limit;
            self
        }

        #[inline]
        pub fn encoded_length(&self) -> usize {
            self.limit - self.offset
        }

        pub fn header(self, offset: usize) -> MessageHeaderEncoder<Self> {
            let mut header = MessageHeaderEncoder::default().wrap(self, offset);
            header.block_length(SBE_BLOCK_LENGTH);
            header.template_id(SBE_TEMPLATE_ID);
            header.schema_id(SBE_SCHEMA_ID);
            header.version(SBE_SCHEMA_VERSION);
            header
        }

        /// REQUIRED enum
        #[inline]
        pub fn message_type(&mut self, value: MessageType) {
            let offset = self.offset;
            self.get_buf_mut().put_u16_at(offset, value as u16)
        }

        /// primitive array field 'symbolID'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: US-ASCII
        /// - semanticType: String
        /// - encodedOffset: 2
        /// - encodedLength: 20
        /// - version: 0
        #[inline]
        pub fn symbol_id(&mut self, value: [u8; 20]) {
            let offset = self.offset + 2;
            let buf = self.get_buf_mut();
            buf.put_bytes_at(offset, value);
        }

        /// primitive field 'dateTime'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 22
        /// - encodedLength: 8
        #[inline]
        pub fn date_time(&mut self, value: i64) {
            let offset = self.offset + 22;
            self.get_buf_mut().put_i64_at(offset, value);
        }

        /// primitive field 'openPrice'
        /// - min value: 1.401298464324817E-45
        /// - max value: 3.4028234663852886E38
        /// - null value: NaN
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 30
        /// - encodedLength: 4
        #[inline]
        pub fn open_price(&mut self, value: f32) {
            let offset = self.offset + 30;
            self.get_buf_mut().put_f32_at(offset, value);
        }

        /// primitive field 'highPrice'
        /// - min value: 1.401298464324817E-45
        /// - max value: 3.4028234663852886E38
        /// - null value: NaN
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 34
        /// - encodedLength: 4
        #[inline]
        pub fn high_price(&mut self, value: f32) {
            let offset = self.offset + 34;
            self.get_buf_mut().put_f32_at(offset, value);
        }

        /// primitive field 'lowPrice'
        /// - min value: 1.401298464324817E-45
        /// - max value: 3.4028234663852886E38
        /// - null value: NaN
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 38
        /// - encodedLength: 4
        #[inline]
        pub fn low_price(&mut self, value: f32) {
            let offset = self.offset + 38;
            self.get_buf_mut().put_f32_at(offset, value);
        }

        /// primitive field 'closePrice'
        /// - min value: 1.401298464324817E-45
        /// - max value: 3.4028234663852886E38
        /// - null value: NaN
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 42
        /// - encodedLength: 4
        #[inline]
        pub fn close_price(&mut self, value: f32) {
            let offset = self.offset + 42;
            self.get_buf_mut().put_f32_at(offset, value);
        }

        /// primitive field 'volume'
        /// - min value: 1.401298464324817E-45
        /// - max value: 3.4028234663852886E38
        /// - null value: NaN
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 46
        /// - encodedLength: 4
        #[inline]
        pub fn volume(&mut self, value: f32) {
            let offset = self.offset + 46;
            self.get_buf_mut().put_f32_at(offset, value);
        }
    }
} // end encoder

pub mod decoder {
    use super::*;

    #[derive(Clone, Copy, Debug, Default)]
    pub struct DataBarDecoder<'a> {
        buf: ReadBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
        pub acting_block_length: u16,
        pub acting_version: u16,
    }

    impl<'a> Reader<'a> for DataBarDecoder<'a> {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            &self.buf
        }
    }

    impl<'a> Decoder<'a> for DataBarDecoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> DataBarDecoder<'a> {
        pub fn wrap(
            mut self,
            buf: ReadBuf<'a>,
            offset: usize,
            acting_block_length: u16,
            acting_version: u16,
        ) -> Self {
            let limit = offset + acting_block_length as usize;
            self.buf = buf;
            self.initial_offset = offset;
            self.offset = offset;
            self.limit = limit;
            self.acting_block_length = acting_block_length;
            self.acting_version = acting_version;
            self
        }

        #[inline]
        pub fn encoded_length(&self) -> usize {
            self.limit - self.offset
        }

        pub fn header(self, mut header: MessageHeaderDecoder<ReadBuf<'a>>) -> Self {
            debug_assert_eq!(SBE_TEMPLATE_ID, header.template_id());
            let acting_block_length = header.block_length();
            let acting_version = header.version();

            self.wrap(
                header.parent().unwrap(),
                message_header_codec::ENCODED_LENGTH,
                acting_block_length,
                acting_version,
            )
        }

        /// REQUIRED enum
        #[inline]
        pub fn message_type(&self) -> MessageType {
            self.get_buf().get_u16_at(self.offset).into()
        }

        #[inline]
        pub fn symbol_id(&self) -> [u8; 20] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset + 2)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn date_time(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset + 22)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn open_price(&self) -> f32 {
            self.get_buf().get_f32_at(self.offset + 30)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn high_price(&self) -> f32 {
            self.get_buf().get_f32_at(self.offset + 34)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn low_price(&self) -> f32 {
            self.get_buf().get_f32_at(self.offset + 38)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn close_price(&self) -> f32 {
            self.get_buf().get_f32_at(self.offset + 42)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn volume(&self) -> f32 {
            self.get_buf().get_f32_at(self.offset + 46)
        }
    }
} // end decoder
