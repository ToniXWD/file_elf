import React, { useState } from 'react';
import { Button, ListGroup } from 'react-bootstrap';


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

    const openVSCode = (filePath) => {
        const invoke = window.__TAURI__.core.invoke;

        invoke('open_vscode', { path: filePath });
        addMessage(`Directory opened in vscode: ${filePath}`);
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
        const invoke = window.__TAURI__.core.invoke;
        try {
            const api_name = isFavorited
                ? 'unstar_path'
                : 'star_path';
            const response = await invoke(api_name, { path });

            if (response) {
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
            <Button variant="outline-secondary" size="sm" onClick={() => openVSCode(result[0])}>
                <img src="/code.ico" alt="VSCode Icon" style={{ width: '16px', height: '16px' }} />
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
