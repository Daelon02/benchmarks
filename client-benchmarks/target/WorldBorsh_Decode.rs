impl :: bincode :: Decode for WorldBorsh
{
    fn decode < __D : :: bincode :: de :: Decoder > (decoder : & mut __D) ->
    core :: result :: Result < Self, :: bincode :: error :: DecodeError >
    { Ok(Self { 0 : :: bincode :: Decode :: decode(decoder) ?, }) }
} impl < '__de > :: bincode :: BorrowDecode < '__de > for WorldBorsh
{
    fn borrow_decode < __D : :: bincode :: de :: BorrowDecoder < '__de > >
    (decoder : & mut __D) -> core :: result :: Result < Self, :: bincode ::
    error :: DecodeError >
    {
        Ok(Self
        { 0 : :: bincode :: BorrowDecode :: borrow_decode(decoder) ?, })
    }
}