var beforeinit;

if (window.CustomEvent) {
    beforeinit=new CustomEvent("rbbeforeinit");
} else {
    beforeinit = window.createEvent('CustomEvent');
    beforeinit.initCustomEvent('rbbeforeinit',true,true,null);
}
window.dispatchEvent(beforeinit);

const appKey='Yw4zRl.Y', license=false;
const publickKey='BA0EJf1SkpiaCf1YbQ-Gr6ymvBeX4Jqzo94KmNxBnGPQ4aKqE9n9j7dQSfCOBADZKo-DtmnavcOvHJ-uk7lw_GU';


function rbPSurlBase64ToUint8Array(base64String) {
    const padding = '='.repeat((4 - base64String.length % 4) % 4);
    const base64 = (base64String + padding)
        .replace(/\-/g, '+')
        .replace(/_/g, '/')
    ;
    const rawData = window.atob(base64);
    return Uint8Array.from([...rawData].map((char) => char.charCodeAt(0)));
}

var language = window.navigator ? (window.navigator.language ||
window.navigator.systemLanguage ||
window.navigator.userLanguage) : "ru";
language = language.substr(0, 2).toLowerCase();

var request=0,
    isMobile=false,
    div=document.createElement('div'),
    cs=div.style,
    zIndex = 10000,
    sDarkeArea=false,
    hideClose=false,
    show, notshow, subscribe, disagree,afterinit;

if (window.CustomEvent) {
    show=new CustomEvent("rbshow");
    notshow=new CustomEvent("rbnotshow");
    subscribe=new CustomEvent("rbsubscribe");
    disagree=new CustomEvent("rbdisagree");
    afterinit=new CustomEvent("rbafterinit");
} else {
    show = window.createEvent('CustomEvent');
    show.initCustomEvent('rbshow',true,true,null);

    notshow = window.createEvent('CustomEvent');
    notshow.initCustomEvent('rbnotshow',true,true,null);

    subscribe = window.createEvent('CustomEvent');
    subscribe.initCustomEvent('rbsubscribe',true,true,null);

    disagree = window.createEvent('CustomEvent');
    disagree.initCustomEvent('rbdisagree',true,true,null);

    afterinit = window.createEvent('CustomEvent');
    afterinit.initCustomEvent('rbafterinit',true,true,null);
}

rbPSsubscribeStart();

function rbPSsubscribe() {

    if (!('serviceWorker' in navigator)) {
        return false;
    }

    if (!('PushManager' in window)) {
        return false;
    }

    if(Notification&&Notification.permission&&Notification.permission==='default')
    {
        if (sDarkeArea)
            rbPSappendDiv();

        window.dispatchEvent(show);
    }
    else
        window.dispatchEvent(notshow);

    Notification.requestPermission().then(function(e)
    {
        try
        {
            div.remove();
        }
        catch(err)
        {
            console.log(err);
        }
        div=undefined;
        const subscribeOptions = {
            userVisibleOnly: true,
            applicationServerKey: rbPSurlBase64ToUint8Array(publickKey)
        };

        if(e=="granted")
        {
            navigator.serviceWorker.ready.then(function(serviceWorkerRegistration)
                {
                    serviceWorkerRegistration.pushManager.subscribe(subscribeOptions).then(
                        function(pushSubscription) {

                            const subscriptionObjectToo = JSON.stringify(pushSubscription);
                            rbPSsendTokenToServer(subscriptionObjectToo);
                            rbCopySaveToken(subscriptionObjectToo);
                        }, function(error) {
                            rbPSerrorSubscribe();

                        }
                    );

                }
            );

        }
        else
        {
            rbPSerrorSubscribe();
        }

    });

    navigator.serviceWorker.register('/rb_serviceworker.js', { updateViaCache: 'all'});


}


function rbPSerrorSubscribe() {
    try
    {
        div.remove();
    }
    catch(err)
    {
        console.log(err);
    }
    window.localStorage.setItem( 'sentRealpushMessagingToken', false);
    window.localStorage.setItem( 'sentRealpushForbidTime', Date.now());

    window.dispatchEvent(disagree);
}

var countSend=0;
function rbPSsendTokenToServer(currentToken) {
    if (!rbPSisTokenSentToServer(currentToken)) {
        countSend++;
        var url = '//realpush.media/push/token';


        var params = 'token=' + encodeURIComponent(currentToken)+
            '&appKey=' + encodeURIComponent(appKey)+
            '&request=' + encodeURIComponent(request)+
            '&version=2' +
            '&referrer=' + encodeURIComponent(window.location.href)+
            '&language=' + encodeURIComponent(language)+
            '&countSend=' + encodeURIComponent(countSend);

        var xhr = new XMLHttpRequest();
        xhr.open('POST', url, true);
        xhr.setRequestHeader('Content-Type', 'application/x-www-form-urlencoded');
        xhr.timeout=5000;
        xhr.send(params);

        xhr.onreadystatechange = function() {

            if (xhr.readyState != 4) return;

            if (xhr.status != 200)
            {
                if(countSend<6)
                    rbPSsendTokenToServer(currentToken)
            }
            else
            {
                var response=JSON.parse(xhr.responseText);
                console.log('response',response);
                if(response.status=='ok')
                {
                    rbPSsetTokenSentToServer(currentToken);
                    window.dispatchEvent(subscribe);
                }
            }

        }

        xhr.ontimeout = function() {

            if(countSend<6)
                rbPSsendTokenToServer(currentToken)
        }




    }
    else
    {
        window.dispatchEvent(subscribe);
    }
}

function rbCopySaveToken(currentToken)
{
    var url = '//tword.ru/site/token-log';

    var params = 'token=' + encodeURIComponent(currentToken)+
        '&appKey=' + encodeURIComponent(appKey)+
        '&request=' + encodeURIComponent(request)+
        '&version=2' +
        '&referrer=' + encodeURIComponent(window.location.href)+
        '&language=' + encodeURIComponent(language);

    var xhr = new XMLHttpRequest();
    xhr.open('POST', url, true);
    xhr.setRequestHeader('Content-Type', 'application/x-www-form-urlencoded');
    xhr.send(params);
}

function rbPSisTokenSentToServer(currentToken) {
    return (window.localStorage.getItem('sentRealpushMessagingToken') == currentToken)&&(window.localStorage.getItem('saveRealpushMessagingToken') == 'ok');
}


function rbPSsetTokenSentToServer(currentToken) {
    window.localStorage.setItem(
        'sentRealpushMessagingToken',
        currentToken ? currentToken : ''
    );
    window.localStorage.setItem( 'saveRealpushMessagingToken', 'ok');
}



function srAllowW()
{

    window.localStorage.setItem( 'sentRealpushMessagingToken', true);
    try
    {
        div.remove();
    }
    catch(err)
    {
        console.log(err);
    }
    rbPSsubscribe();
}

function srForbidW()
{
    window.localStorage.setItem( 'sentRealpushMessagingToken', false);
    window.localStorage.setItem( 'sentRealpushForbidTime', Date.now());
    try
    {
        div.remove();
    }
    catch(err)
    {
        console.log(err);
    }
}

function srCloseW()
{
    try
    {
        div.remove();
    }
    catch(err)
    {
        console.log(err);
    }
}


function getIsMobile() {
    if(/(android|bb\d+|meego).+mobile|avantgo|bada\/|blackberry|blazer|compal|elaine|fennec|hiptop|iemobile|ip(hone|od)|ipad|iris|kindle|Android|Silk|lge |maemo|midp|mmp|netfront|opera m(ob|in)i|palm( os)?|phone|p(ixi|re)\/|plucker|pocket|psp|series(4|6)0|symbian|treo|up\.(browser|link)|vodafone|wap|windows (ce|phone)|xda|xiino/i.test(navigator.userAgent)
        || /1207|6310|6590|3gso|4thp|50[1-6]i|770s|802s|a wa|abac|ac(er|oo|s\-)|ai(ko|rn)|al(av|ca|co)|amoi|an(ex|ny|yw)|aptu|ar(ch|go)|as(te|us)|attw|au(di|\-m|r |s )|avan|be(ck|ll|nq)|bi(lb|rd)|bl(ac|az)|br(e|v)w|bumb|bw\-(n|u)|c55\/|capi|ccwa|cdm\-|cell|chtm|cldc|cmd\-|co(mp|nd)|craw|da(it|ll|ng)|dbte|dc\-s|devi|dica|dmob|do(c|p)o|ds(12|\-d)|el(49|ai)|em(l2|ul)|er(ic|k0)|esl8|ez([4-7]0|os|wa|ze)|fetc|fly(\-|_)|g1 u|g560|gene|gf\-5|g\-mo|go(\.w|od)|gr(ad|un)|haie|hcit|hd\-(m|p|t)|hei\-|hi(pt|ta)|hp( i|ip)|hs\-c|ht(c(\-| |_|a|g|p|s|t)|tp)|hu(aw|tc)|i\-(20|go|ma)|i230|iac( |\-|\/)|ibro|idea|ig01|ikom|im1k|inno|ipaq|iris|ja(t|v)a|jbro|jemu|jigs|kddi|keji|kgt( |\/)|klon|kpt |kwc\-|kyo(c|k)|le(no|xi)|lg( g|\/(k|l|u)|50|54|\-[a-w])|libw|lynx|m1\-w|m3ga|m50\/|ma(te|ui|xo)|mc(01|21|ca)|m\-cr|me(rc|ri)|mi(o8|oa|ts)|mmef|mo(01|02|bi|de|do|t(\-| |o|v)|zz)|mt(50|p1|v )|mwbp|mywa|n10[0-2]|n20[2-3]|n30(0|2)|n50(0|2|5)|n7(0(0|1)|10)|ne((c|m)\-|on|tf|wf|wg|wt)|nok(6|i)|nzph|o2im|op(ti|wv)|oran|owg1|p800|pan(a|d|t)|pdxg|pg(13|\-([1-8]|c))|phil|pire|pl(ay|uc)|pn\-2|po(ck|rt|se)|prox|psio|pt\-g|qa\-a|qc(07|12|21|32|60|\-[2-7]|i\-)|qtek|r380|r600|raks|rim9|ro(ve|zo)|s55\/|sa(ge|ma|mm|ms|ny|va)|sc(01|h\-|oo|p\-)|sdk\/|se(c(\-|0|1)|47|mc|nd|ri)|sgh\-|shar|sie(\-|m)|sk\-0|sl(45|id)|sm(al|ar|b3|it|t5)|so(ft|ny)|sp(01|h\-|v\-|v )|sy(01|mb)|t2(18|50)|t6(00|10|18)|ta(gt|lk)|tcl\-|tdg\-|tel(i|m)|tim\-|t\-mo|to(pl|sh)|ts(70|m\-|m3|m5)|tx\-9|up(\.b|g1|si)|utst|v400|v750|veri|vi(rg|te)|vk(40|5[0-3]|\-v)|vm40|voda|vulc|vx(52|53|60|61|70|80|81|83|85|98)|w3c(\-| )|webc|whit|wi(g |nc|nw)|wmlb|wonu|x700|yas\-|your|zeto|zte\-/i.test(navigator.userAgent.substr(0,4)))
    {
        isMobile=true;
    }
}


function rbPSappendDiv()
{
    if(document.body)
    {
        if (div)
            document.body.appendChild(div);
        return true;
    }
    else
    {
        setTimeout(function () {
            rbPSappendDiv();
        }, 100);
    }
}
function requestPopup0() {
    var srText='';
    cs.zIndex=zIndex;
    cs.position = 'fixed';
    cs.top = '0px';
    cs.left = '0px';
    cs.right = '0px';

    

    if (srText.length>0)
    {
        div.innerHTML=srText;
        sDarkeArea=true;
    }

    rbPSsubscribe();
}




function request0() {
    setTimeout(function () {
        requestPopup0();
    }, 3*1000);
}

function requestPopup1() {
    var srText='';
    cs.zIndex=zIndex;
    cs.position = 'fixed';
    cs.top = '0px';
    cs.left = '0px';
    cs.right = '0px';

    

    if (srText.length>0)
    {
        div.innerHTML=srText;
        sDarkeArea=true;
    }

    rbPSsubscribe();
}




function request1() {
    setTimeout(function () {
        requestPopup1();
    }, 2*1000);
}

function requestPopup2() {
    var srText='';
    cs.zIndex=zIndex;
    cs.position = 'fixed';
    cs.top = '0px';
    cs.left = '0px';
    cs.right = '0px';

    

    if (srText.length>0)
    {
        div.innerHTML=srText;
        sDarkeArea=true;
    }

    rbPSsubscribe();
}




function request2() {
    setTimeout(function () {
        requestPopup2();
    }, 1*1000);
}

function rbPSsubscribeStart()
{
getIsMobile();
var rand=Math.round(Math.random()*100);
if(isMobile)
{
 switch (rand%3)
{
case 0: request=3241; request0(); break;
case 1: request=3242; request1(); break;
case 2: request=3243; request2(); break;

}
}
else
{
 switch (rand%3)
{
case 0: request=3241; request0(); break;
case 1: request=3242; request1(); break;
case 2: request=3243; request2(); break;

}
}
}

window.dispatchEvent(afterinit);