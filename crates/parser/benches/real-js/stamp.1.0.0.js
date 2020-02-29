$(function() {

  //スタンプ通知バーが存在していたら、フッターの下部余白を追加する
  if($("#floatbnr").length){
    $(".footerInner").css("padding-bottom","60px");
  }

  // スタンプ獲得時の通知は8秒表示して消す
  $(".stampNotify--gotStamp").delay(8000).fadeOut(200);

  // スタンプ利用同意通知を閉じたときの非表示処理
  var cPath = location.pathname;
  var time = new Date().getTime();
  var cSaveTime = new Date(time + (60*60*24*1000*14)); //有効期限設定(365日)
      cSaveTime = cSaveTime.toUTCString(); //有効期限を文字列に変換
  var cName = "stampNotify=none" + "; path=" + cPath + "; expires=" + cSaveTime + ";";
  if(document.cookie.indexOf("stampNotify") === -1){
    $(".stampNotify--yetAccept").fadeIn(200);
  }
  $(".stampNotify__close").click(function(){
    $(this).parent().fadeOut(200);
    //ins.beaconClick("floatbnr","bnrclos","0");
    document.cookie = cName;
  });

});
