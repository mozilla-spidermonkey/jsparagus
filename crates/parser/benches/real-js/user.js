var d
if (window.localStorage) {
  var info = window.localStorage.getItem('userInfo')
  if (!!info) {
    d = JSON.parse(info)
  }
}
if (!!d) {
  var html = '<div class="logged"><a href="javascript:void(0)" onclick="togglePopup()">欢迎，' + d.nick_name + '</a><img src="' + d.avatar + '"></div>'
  document.querySelector('.r-info').innerHTML = html;
};