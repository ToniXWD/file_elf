import React from 'react';
import { Button } from 'react-bootstrap';

const ConfigBar = () => {
    const edit_config = () => {
        const invoke = window.__TAURI__.core.invoke;

        invoke('open_file', { name: "base.toml" });
    };

    return (
        <Button
            variant="link"
            onClick={edit_config}
            className="small text-decoration-none"
            style={{
                color: '#007bff',
                fontSize: '14px',
                fontWeight: 'bold',
                padding: '5px',
                transition: 'color 0.3s ease'
            }}
            onMouseEnter={e => e.target.style.color = '#62337a'}
            onMouseLeave={e => e.target.style.color = '#51aafe'}
        >
            Config
        </Button>
    );
};

export default ConfigBar;
