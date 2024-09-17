import React from 'react';
import { Toast } from 'react-bootstrap';

const NotificationToast = ({ showToast, message }) => {
    return (
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
            delay={5000}
            autohide
        >
            <Toast.Header>
                <strong className="mr-auto">Notification</strong>
            </Toast.Header>
            <Toast.Body>{message}</Toast.Body>
        </Toast>
    );
};

export default NotificationToast;
