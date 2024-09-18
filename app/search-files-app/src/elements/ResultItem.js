import React, { useState } from 'react';
import { Button, ListGroup } from 'react-bootstrap';

const ResultItem = ({ result, addMessage }) => {
    const [favorites, setFavorites] = useState([]);

    const openFile = (filePath) => {
        const { invoke } = window.__TAURI__.tauri;
        invoke('open_file', { name: filePath });
        addMessage(`File opened: ${filePath}`);
    };

    const openDir = (filePath) => {
        const { invoke } = window.__TAURI__.tauri;
        invoke('open_dir', { name: filePath });
        addMessage(`Directory opened: ${filePath}`);
    };

    const copyPathToClipboard = async (path) => {
        try {
            await navigator.clipboard.writeText(path);
            addMessage(`Path copied to clipboard: ${path}`);
        } catch (err) {
            addMessage(`Failed to copy path: ${path}`);
        }
    };

    const toggleFavorite = (path) => {
        if (favorites.includes(path)) {
            setFavorites(favorites.filter((p) => p !== path));
        } else {
            setFavorites([...favorites, path]);
            addMessage(`Added to favorites: ${path}`);
        }
    };

    return (
        <ListGroup.Item>
            {result}{' '}
            <Button variant="outline-secondary" size="sm" onClick={() => openDir(result)}>
                📁
            </Button>
            <Button variant="outline-secondary" size="sm" onClick={() => openFile(result)}>
                📄
            </Button>
            <Button variant="outline-secondary" size="sm" onClick={() => copyPathToClipboard(result)}>
                Copy Path
            </Button>
            <Button variant="outline-secondary" size="sm" onClick={() => toggleFavorite(result)}>
                {favorites.includes(result) ? '⭐' : '☆'}
            </Button>
        </ListGroup.Item>
    );
};

export default ResultItem;
