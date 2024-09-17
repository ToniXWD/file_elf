import React, { useState } from 'react';
import { Modal, Button, ListGroup } from 'react-bootstrap';
import ResultItem from './ResultItem.js';

const ResultsListModal = ({ results, addMessage }) => {
    const [show, setShow] = useState(false);

    const handleClose = () => setShow(false);
    const handleShow = () => setShow(true);

    return (
        <>
            <Button variant="primary" onClick={handleShow}>
                Show Results
            </Button>

            <Modal show={show} onHide={handleClose} size="lg">
                <Modal.Header closeButton>
                    <Modal.Title>Results</Modal.Title>
                </Modal.Header>
                <Modal.Body style={{ maxHeight: '400px', overflowY: 'auto' }}>
                    <ListGroup>
                        {results.map((result, index) => (
                            <ResultItem key={index} result={result} addMessage={addMessage} />
                        ))}
                    </ListGroup>
                </Modal.Body>
                <Modal.Footer>
                    <Button variant="secondary" onClick={handleClose}>
                        Close
                    </Button>
                </Modal.Footer>
            </Modal>
        </>
    );
};

export default ResultsListModal;
