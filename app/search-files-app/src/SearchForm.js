import React, { useState, useEffect } from 'react';
import axios from 'axios';
import { Form, Button, ListGroup, Container, Row, Col, Toast } from 'react-bootstrap';

const SearchForm = () => {
    const [entry, setEntry] = useState('');
    const [isPrefix, setIsPrefix] = useState(false);
    const [results, setResults] = useState([]);
    const [favorites, setFavorites] = useState([]); // 新增状态用于收藏路径
    const [message, setMessages] = useState([]); // 新增状态用于记录操作
    const [showToast, setShowToast] = useState(false); // 控制 Toast 的显示状态

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
                addMessage('Search completed successfully.');
            } else {
                throw new Error('Invalid response data');
            }
        } catch (error) {
            console.error('Error searching files:', error);
            alert('An error occurred while searching files.');
            addMessage('Error occurred while searching.');
        }
    };

    const openFile = (filePath) => {
        console.log(`Opening file at: ${filePath}`);
        const { invoke } = window.__TAURI__.tauri
        invoke('open_file', { name: filePath });
        addMessage(`File opened: ${filePath}`);
    };

    const openDir = (filePath) => {
        console.log(`Opening dir at: ${filePath}`);
        const { invoke } = window.__TAURI__.tauri
        invoke('open_dir', { name: filePath });
        addMessage(`Directory opened: ${filePath}`);

    };

    const copyPathToClipboard = async (path) => {
        try {
            await navigator.clipboard.writeText(path);
            addMessage(`Path copied to clipboard: ${path}`);
        } catch (err) {
            addMessage(`Removed from favorites: ${path}`);
        }
    };

    const toggleFavorite = (path) => {
        if (favorites.includes(path)) {
            setFavorites(favorites.filter(p => p !== path));
        } else {
            addMessage(`Added to favorites: ${path}`);
        }
    };

    // 新增方法：添加操作记录
    const addMessage = (message) => {
        setMessages(message);
        setShowToast(true);
        setTimeout(() => setShowToast(false), 5000);
    };

    return (
        <Container className="mt-5  text-start">
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
                                <ListGroup.Item key={index}>
                                    {result}{' '}
                                    <Button
                                        variant="outline-secondary"
                                        size="sm"
                                        onClick={() => openDir(result)}
                                    >
                                        Open Dir
                                    </Button>
                                    <Button
                                        variant="outline-secondary"
                                        size="sm"
                                        onClick={() => openFile(result)}
                                    >
                                        Open File
                                    </Button>
                                    <Button
                                        variant="outline-secondary"
                                        size="sm"
                                        onClick={() => copyPathToClipboard(result)}
                                    >
                                        Copy Path
                                    </Button>
                                    <Button
                                        variant="outline-secondary"
                                        size="sm"
                                        onClick={() => toggleFavorite(result)}
                                    >
                                        {favorites.includes(result) ? 'Unstar' : 'Star'}
                                    </Button>
                                </ListGroup.Item>
                            ))}
                        </ListGroup>
                    </div>
                </Col>
            </Row>
            {/* 使用 Bootstrap Toast 美化通知 */}
            <Toast
                style={{
                    position: 'fixed',
                    bottom: '20px',
                    right: '20px',
                    minWidth: '250px',
                    backgroundColor: '#007bff',
                    color: '#fff',
                    zIndex: 1000
                }}
                show={showToast}
                onClose={() => setShowToast(false)}
                delay={5000}
                autohide
            >
                <Toast.Header>
                    <strong className="mr-auto">Notification</strong>
                </Toast.Header>
                <Toast.Body>{message}</Toast.Body>
            </Toast>
        </Container>
    );
};

export default SearchForm;