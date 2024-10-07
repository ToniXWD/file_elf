import React, { useState, useEffect, useRef } from 'react';
import { Form, Button, Container, Row, Col } from 'react-bootstrap';
import ResultsList from './ResultsList.js';
import NotificationToast from './NotificationToast.js';
import HelpBar from './Help.js';
import ConfigBar from './Config.js';

const SearchForm = () => {
    const [entry, setEntry] = useState('');
    const [isFuzzy, setIsFuzzy] = useState(false);
    const [isRegex, setIsRegex] = useState(true); // 默认开启正则
    const [isSmart, setIsSmart] = useState(false);
    const [results, setResults] = useState([]);
    const [message, setMessage] = useState('');
    const [showToast, setShowToast] = useState(false);

    // 创建输入框的引用
    const inputRef = useRef(null);

    // 聚焦输入框的函数
    const focusInput = () => {
        if (inputRef.current) {
            inputRef.current.focus();
        }
    };

    useEffect(() => {
        setResults([]);

        focusInput();
    }, []);

    const handleSearch = async () => {
        setResults([]);
        const invoke = window.__TAURI__.core.invoke;

        try {
            let response;
            if (isSmart) {
                response = await invoke('hot_search', { entry, isFuzzy, isRegex });
            } else if (isRegex) {
                response = await invoke('regex_search', { entry });
            } else {
                response = await invoke('search', { entry, isFuzzy });
            }

            if (response && Array.isArray(response)) {
                setResults(response);
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
        <Container className="mt-2 text-start">
            <Col md={6} className="text-start">
            </Col>
            <br></br>
            <Row className="justify-content-center">
                <Col md={6}>
                    <h1 className="text-start mb-4">Search Files</h1>
                    <Form onSubmit={(e) => e.preventDefault()}>
                        <Form.Group controlId="entry">
                            <Form.Control
                                type="text"
                                value={entry}
                                onChange={(e) => setEntry(e.target.value)}
                                ref={inputRef} // 绑定引用
                                placeholder="Enter your search query"
                                onKeyDown={(e) => {
                                    if (e.key === 'Enter') {
                                        e.preventDefault(); // 阻止默认行为，避免清空输入框
                                        console.log('Enter pressed');
                                        handleSearch();
                                    }
                                }}
                            />
                        </Form.Group>
                        <br />
                        <div className="d-flex align-items-center gap-3">
                            <Form.Check
                                type="checkbox"
                                label="Fuzzy"
                                checked={isFuzzy}
                                onChange={(e) => setIsFuzzy(e.target.checked)}
                            />
                            <Form.Check
                                type="checkbox"
                                label="Regex"
                                checked={isRegex}
                                onChange={(e) => setIsRegex(e.target.checked)}
                            />
                            <Form.Check
                                type="checkbox"
                                label="Smart Mode"
                                checked={isSmart}
                                onChange={(e) => setIsSmart(e.target.checked)}
                            />
                            <HelpBar></HelpBar>
                            <ConfigBar></ConfigBar
                            >
                            <Button variant="primary" type="button" className="ms-auto"
                                onClick={handleSearch}>
                                Search
                            </Button>
                        </div>
                    </Form>
                    <ResultsList results={results} addMessage={addMessage} />
                </Col>
            </Row>
            <NotificationToast showToast={showToast} message={message} />
        </Container>
    );
};

export default SearchForm;
