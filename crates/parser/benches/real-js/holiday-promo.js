function set_ad_name (ad_name) {
        var ad_keyplacement = ['ad_11283','ad_9393', 'ad_1337'];
        var ad_return_value = [];
 
        for (var k = 0; k < ad_keyplacement.length-1; k++) {
                ad_return_value.push(ad_name+'_'+ad_keyplacement[k])
        }
 
        return ad_return_value;
}
 
page_params.holiday_promo = true;
page_params.holiday_ads_array = set_ad_name('holiday_ad');