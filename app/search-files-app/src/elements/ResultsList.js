import React from 'react';
import {ListGroup} from 'react-bootstrap';
import ResultItem from './ResultItem.js';

const ResultsList = ({ results, addMessage }) => {
    // 移除了模态相关的状态和处理函数

    return (
        <>
            {/* 移除了展示模态按钮 */}

            <ListGroup style={{ maxHeight: '400px', overflowY: 'auto' }}>
                {results.map((result, index) => (
                    <ResultItem key={index} result={result} addMessage={addMessage} />
                ))}
            </ListGroup>
        </>
    );
};

export default ResultsList;