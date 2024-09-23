const HelpBar = () => {
    return <a
        href="https://github.com/ToniXWD/file_elf"
        target="_blank"
        rel="noopener noreferrer"
        className="small text-decoration-none"
        style={{
            color: '#007bff', // 自定义颜色
            fontSize: '14px', // 自定义字体大小
            fontWeight: 'bold', // 加粗
            padding: '5px', // 内边距
            transition: 'color 0.3s ease' // 悬停过渡效果
        }}
        onMouseEnter={e => e.target.style.color = '#0056b3'} // 鼠标移入时改变颜色
        onMouseLeave={e => e.target.style.color = '#007bff'} // 鼠标移出时恢复颜色
    >
        Help
    </a>
}

export default HelpBar;
