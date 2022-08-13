import React from "react";
import Button from "react-bootstrap/Button";
import Col from "react-bootstrap/Col";
import Row from "react-bootstrap/Row";
import Image from "react-bootstrap/Image";

import PropTypes from "prop-types";

import { formatNearAmount } from "near-api-js/lib/utils/format";

/**
 * Room Component
 */
const Room = ({ room, booking }) => {
  // get data from room variable
  const { 
    room_id, 
    owner_id, 
    name, 
    image, 
    beds, 
    description, 
    location, 
    price 
   } = room;

   /**
    * 
    */
   const handleBooking = () => {
    // execute booking function
    booking(room_id, price);
   };

  return (
    <Row style={{ padding: "20px" }}>
      <Col xs={1}></Col>
      <Col xs={2}>
        <Image src={image} alt={name} width='300' fluid />
      </Col>
      <Col xs={4}>
        <h4>{owner_id}</h4>
        <h4>{name}</h4>
        <p>{description}</p>
        <h5>{location}</h5>
      </Col>
      <Col xs={2}>
        <p>Beds</p>
        <h6>{beds}</h6>
      </Col>
      <Col xs={3}>
        <h6>1 night</h6>
        <Button
          variant='outline-dark'
          disabled={!window.accountId}
          onClick={handleBooking}
        >
          Book for {formatNearAmount(price)} NEAR
        </Button>
      </Col>
    </Row>
  );
};

// 引数の型を定義
Room.PrpoTypes = {
  room: PropTypes.instanceOf(Object).isRequired,
  booking: PropTypes.func.isRequired,
};

export default Room;