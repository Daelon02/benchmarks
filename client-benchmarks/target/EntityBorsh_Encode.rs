impl :: bincode :: Encode for EntityBorsh
{
    fn encode < __E : :: bincode :: enc :: Encoder >
    (& self, encoder : & mut __E) -> core :: result :: Result < (), :: bincode
    :: error :: EncodeError >
    {
        :: bincode :: Encode :: encode(& self.string, encoder) ? ; :: bincode
        :: Encode :: encode(& self.number, encoder) ? ; :: bincode :: Encode
        :: encode(& self.number_float, encoder) ? ; :: bincode :: Encode ::
        encode(& self.vec, encoder) ? ; :: bincode :: Encode ::
        encode(& self.bool, encoder) ? ; :: bincode :: Encode ::
        encode(& self.string1, encoder) ? ; :: bincode :: Encode ::
        encode(& self.number1, encoder) ? ; :: bincode :: Encode ::
        encode(& self.number_float1, encoder) ? ; :: bincode :: Encode ::
        encode(& self.vec1, encoder) ? ; :: bincode :: Encode ::
        encode(& self.bool1, encoder) ? ; Ok(())
    }
}