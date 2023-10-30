const invoke_tauri = window.__TAURI__.invoke

let g_battleshipEngine;

// Hide the notification div when it's done animating
document.getElementById('notify')
    .addEventListener('animationend', _evt => {
        document.getElementById('notify')
            .classList.remove('notify-animate');
    });

// Hide the "you win" div and reset the game when it is clicked
document.getElementById('win')
    .addEventListener('click', _evt => {
        document.getElementById('win')
            .style.display = 'none';
        reset();
    })

window.addEventListener('load', () => {
    // Kick things off...
    reset();
})

async function reset() {
    g_battleshipEngine = await invoke_tauri('battleship_engine');
    g_battleshipEngine.game_status.ship_statuses.sort((a, b) =>
        a.size === b.size ? a.base.name.localeCompare(b.name) : a.size - b.size
    );
    console.log(g_battleshipEngine);
    redraw();
}

function removeElementsByName(container, name) {
    for (const element of Array.from(container.getElementsByClassName(name))) {
        container.removeChild(element);
    }
}

async function handleShot(row, col) {
    const notify = document.getElementById('notify');
    if (notify.classList.contains('notify-animate'))
        return; // Don't allow another shot until the notification is complete

    const shipStatus = await takeShot(row, col);
    if (shipStatus) {
        notify.textContent = `${shipStatus.base.name.toUpperCase()} ${(isAfloat(shipStatus) ? "HIT!" : "SUNK!")}`;
        notify.classList.add('notify-animate');

        if (!anyAfloat(g_battleshipEngine.game_status)) {
            const win = document.getElementById('win');
            win.style.display = 'block';
        }
    }
    redraw();
}

function showField() {
    function showTarget(row, col, container) {
        // console.log(`${row},${col}: open`);
        const button = document.createElement('input');
        button.value = '⬜';
        button.style.gridColumn = (col + 1).toString();
        button.style.gridRow = (row + 1).toString();
        button.type = "button";
        button.classList.add("cell");
        button.dataset.row = row.toString();
        button.dataset.col = col.toString();
        button.addEventListener('click', ev => {
            const target = ev.target;
            handleShot(Number(target.dataset.row), Number(target.dataset.col));
        })
        container.appendChild(button);
    }

    function showHitOrMiss(char, row, col, container) {
        const element = document.createElement('span');
        element.textContent = char;
        element.style.gridColumn = (col + 1).toString();
        element.style.gridRow = (row + 1).toString();
        element.classList.add("cell");
        container.appendChild(element);
    }

    const missChar = '️✖';
    const hitChar = '💥';
    const openChar = '.';
    const container = document.getElementById('field-container');
    removeElementsByName(container, 'cell');
    const numRows = g_battleshipEngine.board.length;
    const numColumns = g_battleshipEngine.board[0].length;
    for (let row = 0; row < numRows; ++row) {
        for (let col = 0; col < numColumns; ++col) {
            const c = g_battleshipEngine.board[row][col];
            if (c === openChar || /[a-z]/g.test(c)) {
                showTarget(row, col, container);
            } else {
                showHitOrMiss(/[A-Z]/g.test(c) ? hitChar : missChar, row, col, container);
            }
        }
    }
}

function showStatus() {
    function getStatusString(shipStatus) {
        const hitStatusChar = '🟥';
        const noHitStatusChar = '⬜';
        const result = [];
        let si = 0;
        while (si < shipStatus.hits) {
            result.push(hitStatusChar);
            ++si;
        }
        while (si < shipStatus.base.size) {
            result.push(noHitStatusChar);
            ++si;
        }
        return result.join('');
    }

    const container = document.getElementById('status-container');
    (document.getElementById('shots_fired')).textContent = g_battleshipEngine.game_status.shots.toString();
    (document.getElementById('shots_hit')).textContent = g_battleshipEngine.game_status.hits.toString();
    (document.getElementById('shots_missed')).textContent = g_battleshipEngine.game_status.misses.toString();
    removeElementsByName(container, 'summary');

    let row = 5;
    g_battleshipEngine.game_status.ship_statuses.forEach(shipStatus => {
        const label = document.createElement('span');
        label.style.gridColumn = "1";
        label.style.gridRow = row.toString();
        label.classList.add('summary');
        if (!isAfloat(shipStatus))
            label.classList.add('sunk');
        label.textContent = shipStatus.base.name;
        container.appendChild(label);

        const panel = document.createElement('span');
        panel.style.gridColumn = "2";
        panel.style.gridRow = row.toString();
        panel.classList.add('summary');
        panel.textContent = getStatusString(shipStatus);
        container.appendChild(panel);

        ++row;
    })
}

function redraw() {
    showField();
    showStatus();
}

function isAfloat(shipStatus) {
    console.log(`is ${shipStatus.base.name} afloat?: ${shipStatus.hits < shipStatus.base.size}`);
    return shipStatus.hits < shipStatus.base.size;
}

function anyAfloat(gameStatus) {
    return gameStatus.ship_statuses.some(isAfloat);
}

async function takeShot(row, column) {
    // There's no shared memory in Tauri, so we pass the Battleship Engine to the Rust code, 
    // which it modifies and returns, along with the result of the shot.
    let result = await invoke_tauri('take_shot', { battleshipEngine: g_battleshipEngine, row: row, column: column });
    g_battleshipEngine = result.battleship_engine;
    return result.ship_status;
}
export { };
