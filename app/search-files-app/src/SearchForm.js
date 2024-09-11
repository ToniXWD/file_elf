import React, { useState, useEffect } from 'react';
import axios from 'axios';
import { Form, Button, ListGroup, Container, Row, Col } from 'react-bootstrap';

const SearchForm = () => {
    const [entry, setEntry] = useState('');
    const [isPrefix, setIsPrefix] = useState(false);
    const [results, setResults] = useState([]);

    useEffect(() => {
        // 重新加载页面时清除结果
        setResults([]);
    }, []);

    const handleSearch = async () => {
        try {
            const response = await axios.get('http://127.0.0.1:8000/file_elf/search', {
                params: {
                    entry,
                    is_prefix: isPrefix
                }
            });

            if (response.data && Array.isArray(response.data)) {
                setResults(response.data);
            } else {
                throw new Error('Invalid response data');
            }
        } catch (error) {
            console.error('Error searching files:', error);
            alert('An error occurred while searching files.');
        }
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
                        <br></br>
                        <Form.Group controlId="prefix">
                            <Form.Check
                                type="checkbox"
                                label="Is Prefix?"
                                checked={isPrefix}
                                onChange={(e) => setIsPrefix(e.target.checked)}
                            />
                        </Form.Group>
                        <br></br>
                        <Button variant="primary" onClick={handleSearch}>
                            Search
                        </Button>
                    </Form>
                    <div id="results" className="mt-4">
                        <h2>Results:</h2>
                        <ListGroup>
                            {results.map((result, index) => (
                                <ListGroup.Item key={index}>{result}</ListGroup.Item>
                            ))}
                        </ListGroup>
                    </div>
                </Col>
            </Row>
        </Container>
    );
};

export default SearchForm;