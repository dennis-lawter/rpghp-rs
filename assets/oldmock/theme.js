function changeToLight() {
    setCookieTheme('light');
    changeClass('is-dark', 'is-light');
    changeClass('is-dark-monster', 'is-light-monster');
    let themeButton = document.getElementById('changeTheme');
    if (themeButton) {
        themeButton.innerHTML = "Dark Mode";
        themeButton.onclick = changeToDark;
    }
}

function changeToDark() {
    setCookieTheme('dark');
    changeClass('is-light', 'is-dark');
    changeClass('is-light-monster', 'is-dark-monster');
    let themeButton = document.getElementById('changeTheme');
    if (themeButton) {
        themeButton.innerHTML = "Light Mode";
        themeButton.onclick = changeToLight;
    }
}

function changeClass(fromClass, toClass) {
    let elems = Array.from(document.getElementsByClassName(fromClass));
    for (let i = 0; i < elems.length; i++) {
        let elem = elems[i];
        elem.classList.remove(fromClass);
        elem.classList.add(toClass);
    }
}

function setCookieTheme(mode) {
    document.cookie = "theme=" + mode;
}

function getCookieTheme() {
    let decodedCookie = decodeURIComponent(document.cookie);
    try {
        return decodedData = decodedCookie
            .split('; ')
            .find(row => row.startsWith('theme'))
            .split('=')[1];
    } catch (exception) {
        return 'dark';
    }
}

function getThemeAsClass() {
    return "is-" + getCookieTheme();
}

function initTheme() {
    if (!document.cookie) {
        setCookieTheme("light");
    }

    let theme = getCookieTheme();
    switch (theme) {
        case 'light':
            changeToLight();
            break;
        case 'dark':
        default:
            changeToDark();
            break;
    }
}

window.onload = initTheme;
