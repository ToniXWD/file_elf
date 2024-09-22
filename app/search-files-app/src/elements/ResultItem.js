import React, { useState } from 'react';
import { Button, ListGroup } from 'react-bootstrap';
import axios from 'axios';


const ResultItem = ({ result, addMessage }) => {
    const [favorited, setFavorited] = useState(result[1]);

    const openFile = (filePath) => {
        const invoke = window.__TAURI__.core.invoke;

        invoke('open_file', { name: filePath });
        addMessage(`File opened: ${filePath}`);
    };

    const openDir = (filePath) => {
        const invoke = window.__TAURI__.core.invoke;

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


    const toggleFavorite = async (path, isFavorited) => {
        try {
            const url = isFavorited
                ? 'http://127.0.0.1:6789/file_elf/unstar_path'
                : 'http://127.0.0.1:6789/file_elf/star_path';
            const response = await axios.get(url, { params: { path_data: path } });

            if (response.data) {
                console.log(`${isFavorited ? 'Unstarred' : 'Starred'} successfully`);
                setFavorited(!isFavorited);
            } else {
                console.error(`Failed to ${isFavorited ? 'unstar' : 'star'}`);
            }
        } catch (error) {
            console.error('Error:', error);
        }
    };

    return (
        <ListGroup.Item>
            {/* 解构 result 以提取字符串和布尔值 */}
            {result[0]}{' '}  {/* 这是字符串部分 */}
            <Button variant="outline-secondary" size="sm" onClick={() => openDir(result[0])}>
                📁
            </Button>
            <Button variant="outline-secondary" size="sm" onClick={() => openFile(result[0])}>
                📄
            </Button>
            <Button variant="outline-secondary" size="sm" onClick={() => copyPathToClipboard(result[0])}>
                Copy Path
            </Button>
            <Button variant="outline-secondary" size="sm" onClick={() => toggleFavorite(result[0], result[1])}>
                {/* 根据 result[1] 的值显示星号 */}
                {favorited ? '⭐' : '☆'}
            </Button>
        </ListGroup.Item>
    );
};

export default ResultItem;
