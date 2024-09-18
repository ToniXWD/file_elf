import React, { useState, useEffect } from 'react';
import { Form, Button, Container, Row, Col } from 'react-bootstrap';
import axios from 'axios';
import ResultsList from './ResultsList.js';
import NotificationToast from './NotificationToast.js';

const SearchForm = () => {
    const [entry, setEntry] = useState('');
    const [isFuzzy, setIsFuzzy] = useState(false);
    const [isRegex, setIsRegex] = useState(false);
    const [results, setResults] = useState([]);
    const [message, setMessage] = useState('');
    const [showToast, setShowToast] = useState(false);

    useEffect(() => {
        setResults([]);
    }, []);

    const handleSearch = async () => {
        try {
            let response;
            if (isRegex) {
                response = await axios.get('http://127.0.0.1:6789/file_elf/regex_search', {
                    params: { path: entry }
                });
            } else {
                response = await axios.get('http://127.0.0.1:6789/file_elf/search', {
                    params: { entry, is_fuzzy: isFuzzy }
                });
            }

            if (response.data && Array.isArray(response.data)) {
                setResults(response.data);
                addMessage('Search completed successfully.');
            } else {
                throw new Error('Invalid response data');
            }
        } catch (error) {
            console.error('Error searching files:', error);
            addMessage('Error occurred while searching.');
        }
    };

    const addMessage = (msg) => {
        setMessage(msg);
        setShowToast(true);
        setTimeout(() => setShowToast(false), 5000);
    };

    return (
        <Container className="mt-5 text-start">
            <Row className="justify-content-center">
                <Col md={6}>
                    <h1 className="text-start mb-4">Search Files</h1>
                    <Form>
                        <Form.Group controlId="entry">
                            <Form.Label>Search Entry:</Form.Label>
                            <Form.Control
                                type="text"
                                value={entry}
                                onChange={(e) => setEntry(e.target.value)}
                                placeholder="Enter your search query"
                            />
                        </Form.Group>
                        <br />
                        <Form.Check
                            type="checkbox"
                            label="Is Fuzzy?"
                            checked={isFuzzy}
                            onChange={(e) => setIsFuzzy(e.target.checked)}
                        />
                        <br />
                        <Form.Check
                            type="checkbox"
                            label="Is Regex?"
                            checked={isRegex}
                            onChange={(e) => setIsRegex(e.target.checked)}
                        />
                        <br />
                        <Button variant="primary" onClick={handleSearch}>Search</Button>
                    </Form>
                    <ResultsList results={results} addMessage={addMessage} />
                </Col>
            </Row>
            <NotificationToast showToast={showToast} message={message} />
        </Container>
    );
};

export default SearchForm;
