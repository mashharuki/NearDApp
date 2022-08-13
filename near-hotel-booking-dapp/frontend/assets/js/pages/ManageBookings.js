import React, { useEffect, useState } from "react";
import Button from "react-bootstrap/Button";
import Table from "react-bootstrap/Table";
import {
  get_booking_info_for_owner,
  is_available,
  change_status_to_available,
  change_status_to_stay,
} from "../near/utils";

/**
 * ManageBookings Component
 */
const ManageBookings = () => {
  // state variable  
  const [bookedRooms, setBookedRooms] = useState([]);

  /**
   * get booked room data function
   */
  const getBookedRooms = async () => {
    try {
      setBookedRooms(await get_booking_info_for_owner(window.accountId));
    } catch (error) {
      console.log({error});
    }
  };

  /**
   * change status of room
   * @param {*} room_id 
   * @param {*} check_in_date 
   * @returns 
   */
  const handleCheckIn = async (room_id, check_in_date) => {
    // check room status
    let isAvailable = await is_available(room_id);

    if (isAvailable == false) {
      alert("Error: Someone already stay.");
      return;
    }

    try {
      // call chage_status_to_stay function
      change_status_to_stay(room_id, check_in_date).then((resp) => {
        getBookedRooms();
      });
    } catch (error) {
      console.log({ error });
    }
  };

  /**
   * check out method
   * @param {*} room_id 
   * @param {*} check_in_date 
   * @param {*} guest_id 
   */
  const handleCheckOut = async (room_id, check_in_date, guest_id) => {
    try {
      // call change_status_to_available function
      change_status_to_available(room_id, check_in_date, guest_id).then(
        (resp) => {
          getBookedRooms();
        }
      );
    } catch (error) {
      console.log({ error });
    }
  };

  // hook
  useEffect(() => {
    getBookedRooms();
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
            <th scope='col'>Room Name</th>
            <th scope='col'>Check In</th>
            <th scope='col'>GuestID</th>
            <th scope='col'>Manage Status</th>
          </tr>
        </thead>
        {bookedRooms.map((_room) => (
          <tbody key={_room.room_id + _room.check_in_date}>
            <tr>
              <td>{_room.name}</td>
              <td>{_room.check_in_date}</td>
              <td>{_room.guest_id}</td>
              <td>
                
                {_room.status === "Available" && (
                  <Button
                    variant='success'
                    size='sm'
                    onClick={(e) =>
                      handleCheckIn(_room.room_id, _room.check_in_date, e)
                    }
                  >
                    Check In
                  </Button>
                )}
                
                {_room.status !== "Available" && (
                  <Button
                    variant='danger'
                    size='sm'
                    onClick={(e) =>
                      handleCheckOut(
                        _room.room_id,
                        _room.check_in_date,
                        _room.guest_id,
                        e
                      )
                    }
                  >
                    Check Out
                  </Button>
                )}
              </td>
            </tr>
          </tbody>
        ))}
      </Table>
    </>
  );
}

export default ManageBookings;