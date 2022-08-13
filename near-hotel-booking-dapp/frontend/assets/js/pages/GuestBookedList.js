import React, { useEffect, useState } from "react";
import { Table } from "react-bootstrap";
import { get_booking_info_for_guest } from "../near/utils";

/**
 * GuestBookedList Component
 */
const GuestBookedList = () => {
    // state variable
    const [guestBookedRooms, setGuestBookedRooms] = useState([]);
    
    /**
     * get geust room data
     */
    const getGuestBookedRooms = async () => {
      try {
        setGuestBookedRooms(await get_booking_info_for_guest(window.accountId));
      } catch (error) {
        console.log({error});
      }
    };
  
    // hook
    useEffect(() => {
      getGuestBookedRooms();
    }, []);
  
    if (!window.accountId) {
      return (
        <>
          <h2>Please connect NEAR wallet.</h2>
        </>
      );
    }
    
    return (
      <>
        <h2>BOOKED LIST</h2>
        <Table striped bordered hover>
          <thead>
            <tr>
              <th scope='col'>Owner</th>
              <th scope='col'>Room Name</th>
              <th scope='col'>Check In</th>
            </tr>
          </thead>
          {guestBookedRooms.map((_room) => (
            <tbody key={_room.room_id}>
              <tr>
                <td>{_room.owner_id}</td>
                <td>{_room.name}</td>
                <td>{_room.check_in_date}</td>
              </tr>
            </tbody>
          ))}
        </Table>
      </>
    );
};
  
export default GuestBookedList;