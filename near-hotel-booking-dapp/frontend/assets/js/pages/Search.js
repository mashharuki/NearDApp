import React, { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import Row from "react-bootstrap/Row";
import Room from "../components/Room";
import FormDate from "../components/FormDate";
import { 
    get_available_rooms, 
    book_room 
} from "../near/utils";

/**
 * Search Component
 */
const Search = () => {
  // get data from URL Param
  const { date } = useParams();
  // state variable
  const [availableRooms, setAvailableRooms] = useState([]);

  /**
   * get Available Rooms function
   */
  const getAvailableRooms = async () => {
    // call get_available_rooms function
    setAvailableRooms(await get_available_rooms(date));
  };

  /**
   * booiking function
   * @param {*} room_id Room ID
   * @param {*} price value
   */
  const booking = async (room_id, price) => {
    // execute book room
    book_room({
      room_id,
      date,
      price,
    });

    getAvailableRooms();
  };

  // hook
  useEffect(() => {
    getAvailableRooms();
  }, [date]);

  return (
    <>
      <FormDate />
      <div className='text-center' style={{ margin: "20px" }}>
        <h2>{date}</h2>
        {availableRooms.length === 0 ? (
          <h3>Sorry, no rooms found.</h3>
        ) : (
          <>
            {/* NEAR Walletに接続されている時 */}
            {(window, accountId && <h3>{availableRooms.length} found.</h3>)}
            {/* NEAR Walletに接続していない時 */}
            {!window.accountId && (
              <h3>
                {availableRooms.length} found. To book, you must be connected to
                the NEAR Wallet.
              </h3>
            )}
          </>
        )}
      </div>
      <Row>
        {availableRooms.map((_room) => (
          <Room room={{ ..._room }} key={_room.room_id} booking={booking} />
        ))}
      </Row>
    </>
  );
}

export default Search;