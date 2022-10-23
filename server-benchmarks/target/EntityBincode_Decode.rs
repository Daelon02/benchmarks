impl :: bincode :: Decode for EntityBincode
{
    fn decode < __D : :: bincode :: de :: Decoder > (decoder : & mut __D) ->
    core :: result :: Result < Self, :: bincode :: error :: DecodeError >
    {
        Ok(Self
        {
            string : :: bincode :: Decode :: decode(decoder) ?, number : ::
            bincode :: Decode :: decode(decoder) ?, number_float : :: bincode
            :: Decode :: decode(decoder) ?, vec : :: bincode :: Decode ::
            decode(decoder) ?, bool : :: bincode :: Decode :: decode(decoder)
            ?, string1 : :: bincode :: Decode :: decode(decoder) ?, number1 :
            :: bincode :: Decode :: decode(decoder) ?, number_float1 : ::
            bincode :: Decode :: decode(decoder) ?, vec1 : :: bincode ::
            Decode :: decode(decoder) ?, bool1 : :: bincode :: Decode ::
            decode(decoder) ?,
        })
    }
} impl < '__de > :: bincode :: BorrowDecode < '__de > for EntityBincode
{
    fn borrow_decode < __D : :: bincode :: de :: BorrowDecoder < '__de > >
    (decoder : & mut __D) -> core :: result :: Result < Self, :: bincode ::
    error :: DecodeError >
    {
        Ok(Self
        {
            string : :: bincode :: BorrowDecode :: borrow_decode(decoder) ?,
            number : :: bincode :: BorrowDecode :: borrow_decode(decoder) ?,
            number_float : :: bincode :: BorrowDecode ::
            borrow_decode(decoder) ?, vec : :: bincode :: BorrowDecode ::
            borrow_decode(decoder) ?, bool : :: bincode :: BorrowDecode ::
            borrow_decode(decoder) ?, string1 : :: bincode :: BorrowDecode ::
            borrow_decode(decoder) ?, number1 : :: bincode :: BorrowDecode ::
            borrow_decode(decoder) ?, number_float1 : :: bincode ::
            BorrowDecode :: borrow_decode(decoder) ?, vec1 : :: bincode ::
            BorrowDecode :: borrow_decode(decoder) ?, bool1 : :: bincode ::
            BorrowDecode :: borrow_decode(decoder) ?,
        })
    }
}