import React from "react";
import UploadButton from "../components/UploadButton";

/**
 * Uplaod Component
 */
const Upload = () => {
  return (
    <>
      <div className='text-center' style={{ margin: "200px" }}>
        <h1>You can upload Room image!!</h1>
        <UploadButton/>
      </div>
    </>
  );
}

export default Upload;