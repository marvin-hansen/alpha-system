use crate::*;

pub use decoder::OrderCreateDecoder;
pub use encoder::OrderCreateEncoder;

pub const SBE_BLOCK_LENGTH: u16 = 74;
pub const SBE_TEMPLATE_ID: u16 = 401;
pub const SBE_SCHEMA_ID: u16 = 1;
pub const SBE_SCHEMA_VERSION: u16 = 1;
pub const SBE_SEMANTIC_VERSION: &str = "5.2";

pub mod encoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct OrderCreateEncoder<'a> {
        buf: WriteBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
    }

    impl<'a> Writer<'a> for OrderCreateEncoder<'a> {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            &mut self.buf
        }
    }

    impl<'a> Encoder<'a> for OrderCreateEncoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> OrderCreateEncoder<'a> {
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

        /// primitive field 'exchangeID'
        /// - min value: 0
        /// - max value: 254
        /// - null value: 255
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 2
        /// - encodedLength: 1
        #[inline]
        pub fn exchange_id(&mut self, value: u8) {
            let offset = self.offset + 2;
            self.get_buf_mut().put_u8_at(offset, value);
        }

        /// primitive field 'clientID'
        /// - min value: 0
        /// - max value: 65534
        /// - null value: 65535
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 3
        /// - encodedLength: 2
        #[inline]
        pub fn client_id(&mut self, value: u16) {
            let offset = self.offset + 3;
            self.get_buf_mut().put_u16_at(offset, value);
        }

        /// primitive array field 'clientOrderID'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: US-ASCII
        /// - semanticType: String
        /// - encodedOffset: 5
        /// - encodedLength: 14
        /// - version: 0
        #[inline]
        pub fn client_order_id(&mut self, value: [u8; 14]) {
            let offset = self.offset + 5;
            let buf = self.get_buf_mut();
            buf.put_bytes_at(offset, value);
        }

        /// primitive array field 'symbolID'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: US-ASCII
        /// - semanticType: String
        /// - encodedOffset: 19
        /// - encodedLength: 20
        /// - version: 0
        #[inline]
        pub fn symbol_id(&mut self, value: [u8; 20]) {
            let offset = self.offset + 19;
            let buf = self.get_buf_mut();
            buf.put_bytes_at(offset, value);
        }

        /// REQUIRED enum
        #[inline]
        pub fn order_type(&mut self, value: OrderType) {
            let offset = self.offset + 39;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn order_side(&mut self, value: OrderSide) {
            let offset = self.offset + 40;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn time_in_force(&mut self, value: OrderTimeInForce) {
            let offset = self.offset + 41;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn time_expiry_encoder(self) -> OptionalDecimalEncodingEncoder<Self> {
            let offset = self.offset + 42;
            OptionalDecimalEncodingEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn order_qty_encoder(self) -> DecimalQtyEncoder<Self> {
            let offset = self.offset + 50;
            DecimalQtyEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn order_price_encoder(self) -> DecimalPriceEncoder<Self> {
            let offset = self.offset + 58;
            DecimalPriceEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn order_stop_price_encoder(self) -> OptionalDecimalEncodingEncoder<Self> {
            let offset = self.offset + 66;
            OptionalDecimalEncodingEncoder::default().wrap(self, offset)
        }
    }
} // end encoder

pub mod decoder {
    use super::*;

    #[derive(Clone, Copy, Debug, Default)]
    pub struct OrderCreateDecoder<'a> {
        buf: ReadBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
        pub acting_block_length: u16,
        pub acting_version: u16,
    }

    impl<'a> Reader<'a> for OrderCreateDecoder<'a> {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            &self.buf
        }
    }

    impl<'a> Decoder<'a> for OrderCreateDecoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> OrderCreateDecoder<'a> {
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

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn exchange_id(&self) -> u8 {
            self.get_buf().get_u8_at(self.offset + 2)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn client_id(&self) -> u16 {
            self.get_buf().get_u16_at(self.offset + 3)
        }

        #[inline]
        pub fn client_order_id(&self) -> [u8; 14] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset + 5)
        }

        #[inline]
        pub fn symbol_id(&self) -> [u8; 20] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset + 19)
        }

        /// REQUIRED enum
        #[inline]
        pub fn order_type(&self) -> OrderType {
            self.get_buf().get_u8_at(self.offset + 39).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn order_side(&self) -> OrderSide {
            self.get_buf().get_u8_at(self.offset + 40).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn time_in_force(&self) -> OrderTimeInForce {
            self.get_buf().get_u8_at(self.offset + 41).into()
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn time_expiry_decoder(self) -> OptionalDecimalEncodingDecoder<Self> {
            let offset = self.offset + 42;
            OptionalDecimalEncodingDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn order_qty_decoder(self) -> DecimalQtyDecoder<Self> {
            let offset = self.offset + 50;
            DecimalQtyDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn order_price_decoder(self) -> DecimalPriceDecoder<Self> {
            let offset = self.offset + 58;
            DecimalPriceDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn order_stop_price_decoder(self) -> OptionalDecimalEncodingDecoder<Self> {
            let offset = self.offset + 66;
            OptionalDecimalEncodingDecoder::default().wrap(self, offset)
        }
    }
} // end decoder
