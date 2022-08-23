import { useState } from 'react';
import Button from "react-bootstrap/Button";
import Form from 'react-bootstrap/Form';
import Spinner from 'react-bootstrap/Spinner';
import axios from 'axios';
import FormData from 'form-data';

// API Endpoint
const baseAPIUrl = "https://api.pinata.cloud";

// read from .env
const {
    PINATA_API_Key,
    PINATA_API_Secret
} = process.env;

/**
 * UploadButton Component
 */
const UploadButton = () => { 
    // state variable
    const [ fileName, setFileName ] = useState('select a file');
    const [ file, setFile] = useState({});
    const [ pendingFlg, setPendingFlg ] = useState(false);

    /**
     * ファイル名とファイル本体を保存するための関数
     */
    const saveFile = (e) => {
        setFile(e.target.files[0]);
        setFileName(e.target.files[0].name);
    };

    /**
     * post to pinata Button
     * @param {*} event param
     */
    const pintaUploadFile = async (event) => {
        let postData = new FormData();
        // get param data
        const { name, files } = event.target;
        // create request param data
        postData.append('file', file);
        postData.append('pinataOptions', '{"cidVersion": 1}');
        postData.append('pinataMetadata', `{"name": "${fileName}", "keyvalues": {"company": "nearHotel"}}`);

        try {
            // Flg on
            setPendingFlg(true);
            // POSTメソッドで送信
            const res = await axios.post(
                // API Endpoint URL
                baseAPIUrl + '/pinning/pinFileToIPFS', 
                // param
                postData , 
                // header
                {
                    headers: {
                        'accept': 'application/json',
                        'pinata_api_key': `${PINATA_API_Key}`,
                        'pinata_secret_api_key': `${PINATA_API_Secret}`,
                        'Content-Type': `multipart/form-data; boundary=${postData}`,
                    },  
                });
            console.log(res);
            console.log("CID:", res.data.IpfsHash);
            setPendingFlg(false);
            alert(`upload Successfull!! CID:${res.data.IpfsHash}`);
        } catch (e) {
            console.error("upload failfull.....：", e);
            alert("upload failfull.....");
        }
    }

    return (
        <>
            {pendingFlg ? (
                <Spinner animation="border" role="status">
                    <span className="visually-hidden">Please wait...</span>
                </Spinner>
            ):(
                <>
                    <Form.Group 
                        controlId="formFile" 
                        className="mb-3" 
                        onChange={(e) => saveFile(e)}
                    >
                        <Form.Label>Please drop or select</Form.Label>
                        <Form.Control type="file" />
                    </Form.Group>
                    <Button 
                        onClick={(e) => pintaUploadFile(e)} 
                        variant='info'
                    >
                        Upload Image
                    </Button>
                </>
            )}
        </>
    );
}

export default UploadButton;