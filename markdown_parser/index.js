const rust = import ('./pkg/markdown_parser.js');

rust.then((func)=>{
    const btn = document.getElementById('parse');
    const previewArea = document.getElementById('output');

    btn.addEventListener('click',()=>{
        const input = document.getElementById('markdown').value;
        previewArea.innerHTML = func.parse(input);
    })
})
