<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_dojo.registerModulePath(tapestry, insp_118d36</name>
   <tag></tag>
   <elementGuidId>e0c1447e-1cda-44e9-97fd-f61ba4ab69e7</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//body[@id='Body']</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>#Body</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
      <webElementGuid>7943f8ae-d08c-4c16-b446-07ee5ced30b6</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>leftmargin</name>
      <type>Main</type>
      <value>0</value>
      <webElementGuid>1ee64f35-9fdb-4ff4-bcbd-8727276395ad</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>topmargin</name>
      <type>Main</type>
      <value>0</value>
      <webElementGuid>3179fd78-54bb-4d22-8900-3b2ba9a6cdb7</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>rightmargin</name>
      <type>Main</type>
      <value>0</value>
      <webElementGuid>f2db903c-e3eb-42c9-9e23-a1ff6235ad72</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>bottommargin</name>
      <type>Main</type>
      <value>0</value>
      <webElementGuid>2e70916a-91bd-415c-9610-c1fda8a9c329</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>bgcolor</name>
      <type>Main</type>
      <value>#ffffff</value>
      <webElementGuid>56b54b47-dec2-4961-b5be-e3f1c2dfe64a</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>Body</value>
      <webElementGuid>f1e30ff3-2d46-482d-be3d-9efea214d904</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>

dojo.registerModulePath(&quot;tapestry&quot;, &quot;/inspireapp/assets/static/tapestry-4.1.6&quot;);



dojo.require(&quot;tapestry.namespace&quot;);
tapestry.requestEncoding='UTF-8';















































&lt;!--
// directory of where all the images are
var cmThemePanelBase = '/inspireapp/ThemePanel/';

var cmThemePanel =
{

  	// main menu display attributes
  	//
  	// Note.  When the menu bar is horizontal,
  	// mainFolderLeft and mainFolderRight are
  	// put in &lt;span>&lt;/span>.  When the menu
  	// bar is vertical, they would be put in
  	// a separate TD cell.

  	// HTML code to the left of the folder item
  	mainFolderLeft: '&lt;img alt=&quot;&quot; src=&quot;' + cmThemePanelBase + 'blank.gif&quot;/>',
  	// HTML code to the right of the folder item
  	mainFolderRight: '&lt;img alt=&quot;&quot; src=&quot;' + cmThemePanelBase + 'arrowdown.gif&quot;/>',
	// HTML code to the left of the regular item
	mainItemLeft: '&lt;img alt=&quot;&quot; src=&quot;' + cmThemePanelBase + 'blank.gif&quot;/>',
	// HTML code to the right of the regular item
	mainItemRight: '&lt;img alt=&quot;&quot; src=&quot;' + cmThemePanelBase + 'blank.gif&quot;/>',

	// sub menu display attributes

	// HTML code to the left of the folder item
	folderLeft: '&lt;img alt=&quot;&quot; src=&quot;' + cmThemePanelBase + 'blank.gif&quot;/>',
	// HTML code to the right of the folder item
	folderRight: '&lt;img alt=&quot;&quot; src=&quot;' + cmThemePanelBase + 'arrow.gif&quot;/>',
	// HTML code to the left of the regular item
	itemLeft: '&lt;img alt=&quot;&quot; src=&quot;' + cmThemePanelBase + 'blank.gif&quot;/>',
	// HTML code to the right of the regular item
	itemRight: '&lt;img alt=&quot;&quot; src=&quot;' + cmThemePanelBase + 'blank.gif&quot;/>',
	// cell spacing for main menu
	mainSpacing: 0,
	// cell spacing for sub menus
	subSpacing: 0,
	// auto dispear time for submenus in milli-seconds
	delay: 500
};

// for sub menu horizontal split
var cmThemePanelHSplit = [_cmNoClick, '&lt;td colspan=&quot;3&quot; style=&quot;height: 5px; overflow: hidden&quot;>&lt;div class=&quot;ThemePanelMenuSplit&quot;>&lt;/div>&lt;/td>'];
// for vertical main menu horizontal split
var cmThemePanelMainHSplit = [_cmNoClick, '&lt;td colspan=&quot;3&quot; style=&quot;height: 5px; overflow: hidden&quot;>&lt;div class=&quot;ThemePanelMenuSplit&quot;>&lt;/div>&lt;/td>'];
// for horizontal main menu vertical split
var cmThemePanelMainVSplit = [_cmNoClick, '|'];






function restore(idx) {
		var element = document.getElementById(&quot;restore&quot;);
		var href = element.href+&quot;&amp;sp=l&quot;+idx;
		tapestry.linkOnClick(href,'restore', false);
    }  
	var clearUrl=null;
    function deleteBibliographic(idx, warning) {
		var element = document.getElementById(&quot;deleteBibliographic&quot;);
		if (clearUrl == null) {
			clearUrl= element.href;
		}
		element.href = clearUrl+&quot;&amp;sp=l&quot;+idx+&quot;&amp;sp=&quot;+warning;
		clickDirectLink(&quot;deleteBibliographic&quot;);
    }
dojo.require(&quot;tapestry.event&quot;);




function openDialogComponent(compId, hideCloseIcon, hideMaximizeIcon, closeHiddenForm) {
  	if(closeHiddenForm!=null &amp;&amp; !String(closeHiddenForm)==''){
	    if(compId) {
	      var d = new MyAppAlert(compId, hideCloseIcon, hideMaximizeIcon, '/inspireapp/images/', closeHiddenForm);
	    }
    }else{
    	if(compId) {
	      var d = new MyAppAlert(compId, hideCloseIcon, hideMaximizeIcon, '/inspireapp/images/');
	    }
    }
  }
  function closeDialogComponent(compId) {
    if(compId) {
        MyAppAlert.closeDialog(compId);
    }
  }
function printItems(href) {
	hrefparameters = addCheck();
	if (hrefparameters!=null){

		popupwindow = window.open(&quot;&quot; ,&quot;printRecord&quot;, &quot;status=no,toolbar=0,scrollbars=yes,menubar=0,titlebar=0,resizable=0,width=1500,height=1500&quot;);
		popupwindow.moveTo(screen.width/2-325, screen.height/2-235);
		popupwindow.focus();
		popupwindow.location = href+hrefparameters;
	}
	else{
		var buttonObject = window.parent.document.getElementById(&quot;InfoMsg&quot;);
	  	buttonObject.onclick();
	}
	return false;
}

function addCheck(){
	var formObject = document.getElementById(&quot;searchForm&quot;);
	total=0;
	href=&quot;&quot;;
	for(var i=0;i &lt; formObject.length;i++) {
		var item = formObject.elements[i];
		if (item.name.indexOf(&quot;selectat&quot;) == 0) {
			if (item.checked == true) {
					id = formObject.elements[i-1];
					if (total == 0) delimitator = &quot;?&quot;;
					else delimitator = &quot;&amp;&quot;;
					href = href+delimitator+&quot;sp=&quot;+id.value;
					total++;					
			}
		}
	}
	if (total>0) return href; 
	else return null;
}

 
 
 function attachFocus(fieldName) {
    var field = document.getElementById(fieldName);
    field.focus();
    VirtualKeyboard.attachInput(field);
}
  
 
  imagesPath = '/inspireapp/images/';
  
   function clickSearchButton() {
    var searchDiv = document.getElementById(&quot;HiddenSearch&quot;);
    var browseDiv = document.getElementById(&quot;HiddenBrowse&quot;);
    if (searchDiv) {
     if (browseDiv) {
	    if (searchDiv.style.display == &quot;none&quot;) {
	      var browseButton = document.getElementById(&quot;browse&quot;);
	      browseButton.click();
	    }
	    else {
		    var searchButton = document.getElementById(&quot;formSubmitSearch&quot;);
		    searchButton.click();
	    }
     }
    }
    else {
     if (browseDiv) {
          var browseButton = document.getElementById(&quot;browse&quot;);
	      browseButton.click();
     }
     else {
		    var searchButton = document.getElementById(&quot;formSubmitSearch&quot;);
		    searchButton.click();
     } 
    }
  }


function changeScheduledStatusReport(){
	var checkBox = document.getElementById(&quot;scheduledReport&quot;);
	var scheduledData = document.getElementById(&quot;ScheduledData&quot;);
	var sheduleImg = document.getElementById(&quot;sheduleImg&quot;);
	
	if (checkBox.checked==false) {
	    checkBox.checked = true;
	    scheduledData.style.display ='';
	    sheduleImg.src =  &quot;images/arrow_checked.gif&quot;;
	  }
	else {
	 checkBox.checked = false;
	 scheduledData.style.display ='none';
	 sheduleImg.src =  &quot;images/arrow_unchecked.gif&quot;;
   }
}
dojo.require(&quot;tapestry.fx&quot;);
// -->


	
	
	







		
	
	
		
			
				
				Hi, catc (CU)
				
				最近登入:2024-03-22 15:26:59 ,成功
				
				
					
					
                       0 筆新訊息
                    
                
				
				繁體中文English繁體中文
				
English
繁體中文
						
						
				
				
					
					登 出
				
			
		
		
		
			
			
				神資圖書館(15trunk機)
			
			
		
	
	
	
	
	
		
		
		
			
			
			 查詢  編目  流通  採購   期刊控制   場地設備  說明  我的帳戶  學科服務  管理  統計  清單  Portal  臉辨報表  ERM 
			簡單查詢進階查詢FRBR查詢最近的查詢已儲存的記錄FRBR作品書目書目記錄維護註記刪除之書目記錄書目記錄全域修改權威記錄權威記錄維護註記刪除之權威記錄權威記錄全域修改條碼號館藏資料維護註記刪除之館藏資料維護出版者出版者資料維護內部移送作業書單維護FRBR作品辦證讀者記錄維護檢查讀者註記刪除之讀者記錄讀者證卡記錄維護離職離校借閱檯讀者流通檯還書箱讀者查詢交易紀錄違規讀者查詢讀者個人違規記錄指定參考書交易歷史查詢一般搜尋交易館藏預約順位調整題名預約順位調整批次修改到期日內部移送作業盤點盤點維護跨館預約/調閱跨館預約/調閱需求移轉-寄送項目移轉-收到項目盤點比對批次報廢選擇報廢執行報廢已報廢歷史查詢書箱作業條碼號空號清單(條碼機印製)訂購檢查薦購請購訂購明細訂單登收用卷期登收程序發票款項維護預算經費供應商標案內部移送作業催缺候選催缺項目催缺記錄期刊主檔訂購檢查薦購請購核可訂購明細訂單登收用卷期登收程序發票款項維護預算經費供應商裝訂裝訂主檔維護擬裝設定送裝確認裝訂登收標案內部移送作業催缺候選催缺項目催缺記錄流通記錄預約記錄借用記錄維護分類維護單項維護參數設定流通政策維護預約審核場地設備黑名單書籤訊息排程工作日誌學科館藏館藏資料類型設定學科館藏維護學科預約教師資料維護讀者評價選單學科預約維護學科預約查詢教師評語學科報表學科館藏冊數統計教師學科服務時數與使用人次統計學科服務讀者清單學科報表學科館藏冊數統計教師學科服務時數與使用人次統計學科服務讀者清單一般設定類型/使用者(館員)片語設定系統設定貨幣匯率館藏地範本設定--依館員帳號翻譯轉換 選取清單格式屬性條碼號規範顯示設定圖書館系統屬性推薦範本設定館員新增申請QR Code 設定編目書目類型設定MARC規範格式設定顯示設定停用字SDI維護同義字條碼號原則對應流通流通通知單範本流通通知單參數設定流通通知單發送紀錄日常作業排程一般每周開閉館設定閉館日期維護交易紀錄類型交易方式可適用讀者身份交易方式可適用館藏流通類別交易方式可適用使用類別交易紀錄-館藏狀態流通政策讀者證卡原則單位所系夾書單欄位設定讀者閱讀興趣發送紀錄期刊週期傳閱清單維護 轉入Z39屬性設定更新書封上傳書封索引編修索引重建功能維護公佈欄維護意見回覆維護主題維護特別書籍維護心得分享東區互惠審核東區互惠匯出系統代碼維護貨幣代碼作者號維護科特號維護OPACSDI維護讀者評論管理Help維護熱門關鍵字過濾館員Log記錄館員登入/出記錄館員資料變更記錄讀者資料變更記錄權限設定MARC權限設定功能權限設定角色設定系統狀態排程管理排程監控者排程日誌查詢統計檢索點/檢索語統計報表不同時段使用次數統計書目下載次數統計電子資源統計電子資源-館藏點閱次數統計電子資源-讀者點閱次數統計電子資源-單位系所點閱次數統計編目統計館藏量統計報表類號區間統計報表圖書增減統計報表編目/館別/大類交叉分析報表大類/冊數統計表大類/特藏交叉統計表特藏/冊數統計表新書特藏大類交叉統計圖書增減表／月份／大類大類語言種數冊數統計大類/條碼號類別冊數統計圖書冊數分類統計單圖書冊數金額分類統計非書資料數量金額統計資料類型範圍館藏統計表編目工作量統計(個人)編目工作量統計(各館)館藏狀態統計報表流通統計讀者借閱排行榜書展借閱清單流通人數統計(學制分頁 符合科系)讀者類型借閱暨歸還流通量統計報表流通人次冊數統計  利用概況統計(日報表)讀者借閱交叉分析各館特藏預約統計報表館內使用統計讀者身份借閱統計讀者個人及單位所系借閱排行榜館藏資料借閱統計單位系所借閱排行榜單位系所借閱統計熱門借閱排行榜熱門預約排行榜讀者辦証暨補發人次統計表借閱交叉分析統計資料類型流通統計館員流通量統計單位(個人)保管圖書(非書)數量明細館際互借代還統計-本館館藏館際互借代還統計-他館館藏指定參考書借閱量統計表各大類熱門借閱排行榜大專院校圖書館讀者身份借閱清單及統計報表館藏流通量統計跨館預約/調閱館藏借閱排行榜罰款收款人數/次數統計跨館預約/調閱使用量統計罰款收款統計特定館藏借閱統計採購統計書商採購統計年度經費採購統計特藏訂購方式金額統計書商到書率書商績效統計期刊統計期刊到刊率統計報表平均收刊日統計報表歷年期刊借閱統計書商到書率書商催缺次數書商累計訂購次數期刊訂費漲幅統計逾期違約金統計期刊經費訂購統計報表場地設備場地設備使用統計表報表模組範本維護編目清單書目標籤報表Tag856檢查報表新版書標維護次分類交叉分析登錄簿條碼產生清單登錄簿清單條碼號空號清單單位科系館藏設定清單館藏狀態預約清單特殊館藏清單館藏狀態清單報表流通清單罰款未收款報表預約待取清單借閱(逾期)清單還書清單新書清單讀者欠款清單借閱逾期清單罰款收款清單預約撤架清單長期借閱圖書清單離線流通清單預約額滿資料清單單位系所借閱逾期清單跨館還書清單離職離校清單跨館移送清單預約保留結束清單指定參考書借閱量明細表未外借館藏清單重複讀者比對報表借閱逾期通知單取書櫃上架清單預約圖書清單採購清單年度經費採買書籍運用率訂購資料明細表擬購資料明細表請購單詢價單訂購逾期清單謝函書的催缺報表薦購書刊複本比對報表採購參考清單財產增加清冊財產減少清冊已付款未到清單訂購明細期刊清單期刊合訂本登錄簿交贈期刊清單期刊架位清單裝訂歷史清單薦購書刊複本比對報表期刊登錄簿索贈清單期刊查驗清單已付款未到清單場地設備預約取消清單臉辨用戶使用分析表臉辨用戶與圖書分類關係表期間辦證統計_年報期間辦證統計_月報期間辦證明細期間辦證異動統計_年報期間辦證異動統計_月報辦證異動明細臉辨使用紀錄統計_年報臉辨使用紀錄統計_月報臉辨使用紀錄明細人數統計表人數統計明細表人次統計表人次統計明細表熱門時段統計表讀者進出排行榜期間辦證刪除明細系統參數設定IP區間管理延伸查詢設定系統設定檔資源瀏覽/後分類設定資源到期通知設定可使用身分類別設定 代碼管理所屬資料庫清單共用代碼檔共用代碼類別電子資源管理資源前端顯示設定Ezproxy設定檔維護電子資料庫/網路資源電子期刊/電子書電子資源批次匯入電子資源副檔匯入讀者回報資源連線異常 Meniuri.erm.9000報表點閱資源排行榜依學院別使用統計依單位別使用統計依月分資源使用統計依學院登入統計表依系所登入統計表依身分登入統計表登入清單依身分別使用統計表使用者使用資源清單 收藏資源排行榜偵測電子資源狀態設定偵測資源時間設定偵測電子資源查詢電子資源狀態電子資源狀況報表
				
	


		
			
				場地設備 > 場地設備黑名單		
			
	


							
	jQuery(function () {
		jQuery(&quot;div[id='hiddenDiacritics']&quot;).draggable({
	        containment: &quot;#box&quot;,
	        scroll: false
	});
	});
	jQuery(function () {
		jQuery(&quot;div[id='symb_div']&quot;).draggable({
			containment: &quot;#box&quot;,
			scroll: false
	});
	});


function checkElement(elementId, checkedElem){
	href = document.getElementById(&quot;checkInform&quot;).href+'&amp;sp=l'+elementId+'&amp;sp='+(checkedElem==true?'T':'F');
	return tapestry.linkOnClick(href,'checkInform', false);
}

function syncWithOrder(){
document.getElementById(&quot;orderCriteria&quot;).value = document.getElementById(&quot;browseCriteria&quot;).value;
}
function syncWithBrowse(){
document.getElementById(&quot;browseCriteria&quot;).value = document.getElementById(&quot;orderCriteria&quot;).value;
}

var refreshTime = 0;
function apasa() {
  refreshTime = 2500;

  clickLinkSubmit(&quot;searchForm&quot;, &quot;scriptSubmit&quot;);
}

function showingOrderBy(status){
	document.getElementById(&quot;showOrderBy&quot;).style.display = status;
}
function remakeSearch(){
	if(typeof(tapestry.form)!=&quot;undefined&quot;) clickLinkSubmit('searchForm', 'remakeSearch');
	else window.setTimeout(&quot;remakeSearch()&quot;,500);
}

function remakePaginatedSearch(){
	clickLinkSubmit('searchForm', 'remakePaginatedSearch');
}
j(document).ready(function(){
	j(&quot;#resetbutton&quot;).click(function(){	
		//j(&quot;#reseter&quot;).click();
		var h=j(&quot;#reseter&quot;).attr(&quot;href&quot;);
		window.location=h;
	});
	
    // JSON 資料結構模擬  start
    try{
    var treeJsonData = JSON.parse(j('#locData').text());
    
    j('.place').treeoptions({
        data: treeJsonData,
        openImg: '/inspireapp/images/ico_add.gif', // img 路徑
        cleanImg:'/inspireapp/images/clear.gif' // img 路徑
    });
    }catch(e){
    }
    try{
    var treeJsonData1 = JSON.parse(j('#itemClassData').text());
    
    j('.place0').treeoptions({
        data: treeJsonData1,
        openImg: '/inspireapp/images/ico_add.gif', // img 路徑
        cleanImg:'/inspireapp/images/clear.gif' // img 路徑
    });
    }catch(e){
    }
    try{
    var treeJsonData2 = JSON.parse(j('#materialTypeData').text());
    
    j('.place1').treeoptions({
        data: treeJsonData2,
        openImg: '/inspireapp/images/ico_add.gif', // img 路徑
        cleanImg:'/inspireapp/images/clear.gif' // img 路徑
    });
    }catch(e){
    }
    try{
        var treeJsonData3 = JSON.parse(j('#languageData').text());
        
        j('.place2').treeoptions({
            data: treeJsonData3,
            openImg: '/inspireapp/images/ico_add.gif', // img 路徑
            cleanImg:'/inspireapp/images/clear.gif' // img 路徑
        });
    }catch(e){
    }
    try{
        var treeJsonData4 = JSON.parse(j('#langCataData').text());
        
        j('.place3').treeoptions({
            data: treeJsonData4,
            openImg: '/inspireapp/images/ico_add.gif', // img 路徑
            cleanImg:'/inspireapp/images/clear.gif' // img 路徑
        });
    }catch(e){
    }
    
    try{
        var departmentJson = JSON.parse(j('#departmentData').text());
        
        j('.multipleDepartmentClass').treeoptions({
            data: departmentJson,
            openImg: '/inspireapp/images/ico_add.gif',
            cleanImg:'/inspireapp/images/clear.gif'
        });
    }catch(e){}
});

function runScript(e) {
    if (e.keyCode == 13) {
        document.getElementById(&quot;browse&quot;).click();
        return false;
    }
}

function createPopEdit(href) {

	popupwindow = window.open(&quot;&quot; ,&quot;MeniuCatalogare&quot;, &quot;status=no,toolbar=0,scrollbars=yes,menubar=0,titlebar=0,resizable=1,width=1500,height=1500&quot;);
	popupwindow.moveTo(screen.width/2-435, screen.height/2-300);
	popupwindow.focus();

	popupwindow.location = href;

	if (popupwindow == null) popupwindow.opener = self;
	return false;

}

function createUploadPopEdit(href) {
	popupwindow = window.open(&quot;&quot; ,&quot;Upload&quot;, &quot;status=no,toolbar=0,scrollbars=yes,menubar=0,titlebar=0,resizable=1,top=&quot; + (screen.height-150)/2 + &quot;,left=&quot; + (screen.width-600)/2 + &quot;,width=600,height=250&quot;);
	popupwindow.focus();

	popupwindow.location = href;

	if (popupwindow == null) popupwindow.opener = self;
	return false;
};



	
	
		
			refreshTime = 0;
		
	



			 
				
						
						查詢
							
						
						
				   
				
						
							瀏覽
								
							
						
				
				
	  		
					
						
						 
							新增 
		
				
				
				
				
				
				
				
				
				
				















































	

	
	
	
	
		
			
			查詢條件
				
					
日期
讀者姓名
讀者證號

				
					
起始以
包含
等於(=)
大於(>=)
小於(&lt;=)
不等於(≠)

				
				
				
					
					
						
						
							
						
					
				
				
				
				
					
and
or
not

					
日期
讀者姓名
讀者證號

				
					
起始以
包含
等於(=)
大於(>=)
小於(&lt;=)
不等於(≠)

				
				
				
					
					
						
						
							
						
					
				
				
				
				
				
				
					
								
		                    
		                    	
								
				
			
			
				
				
				  

					
				

             
			
			
			
				
		
	

	
	
	
		瀏覽條件:
		
			
日期
讀者姓名
讀者證號

		
		
			
			
			
			起始以
			
			:
		
		
			
			
				 
				
				

//&lt;![CDATA[

					document.getElementById(&quot;listField&quot;).focus();
				
//]]&gt;


			
		
	    
		 
			
      
	
	 
	
	
    
		            
		        
		              

		          
			 

			
			
  					
					 
					     
					      
					       
					       
					       
					 		
						                  排序條件 :  
							  
							  
日期
讀者姓名
讀者證號

						       
						       
						      
						      
						   
							
								
升冪
降冪

							 
							
								
								
								



                            	
                              		
                            	
                            	
							
						
						

					 
					 
					   
         
			 
			 
			 
             
                
                    
                      
                        符號表:
                        
                      
                      
                        
貨幣：
￥   
￡   
₤
₣
₢      
₡
₠
₥
₦
₧ 
₨
₩
₪
₫
€

數學符號：
±
Ω
λ
β
α
θ
π
μ
≠
≤
≥
∑

日文：
あ
い
う
え
お
か
き
く
け
こ
さ
し

す
せ
そ
た
ち
つ
て
と
な
に
ぬ
ね

の
は
ひ
ふ
へ
ほ
ま
み
む
め
も
や

ゆ
よ
ら
り
る
れ
ろ
わ
を
ん

が
ぎ

ぐ
げ
ご
ざ
じ
ず
ぜ
ぞ
だ
ぢ
づ
で

ど
ば
び
ぶ
べ
ぼ

ぱ
ぴ
ぷ
ぺ
ぽ


ア
イ
ウ
エ
オ
カ
キ
ク
ケ
コ
サ
シ

ス
セ
ソ
タ
チ
ツ
テ
ト
ナ
ニ
ヌ
ネ

ノ
ハ
ヒ
フ
ヘ
ホ
マ
ミ
ム
メ
モ
ヤ

ユ
ヨ
ラ
リ
ル
レ
ロ
ワ
ヲ
ン
ガ
ギ

グ
ゲ
ゴ
ザ
ジ
ズ
ゼ
ゾ
ダ
ヂ
ヅ
デ

ド
バ
ビ
ブ
ベ
ボ
パ
ピ
プ
ペ
ポ

ぃ
ぅ
ぇ
ぉ
っ
ゃ
ゅ
ょ
ゎ
ァ
ィ
ゥ

ェ
ォ
ッ
ャ
ュ
ョ
ヮ
々

羅馬數字：
Ⅰ
Ⅱ
Ⅲ
Ⅳ
Ⅴ
Ⅵ
Ⅶ
Ⅷ
Ⅸ
Ⅹ


                      
                    
                
                
             
			 
			 
  

  
			
		
		
		 
10
20
50
100

      	 
      	 
		 
		
	
  
  	

	
     	 
		



	

	

		
	
		 


 
 
 
 



 
  
    Go To Page
  
  
 
 
  
 




		  
		    
		  
		  
		  



		  {
  &quot;data&quot; : [  
{
  &quot;name&quot; : &quot;CMUL - 神資圖書館&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;2 - 2&quot;},
{ &quot;name&quot; : &quot;123 - 123&quot;},
{ &quot;name&quot; : &quot;20230417 - 20230417&quot;},
{ &quot;name&quot; : &quot;20230418 - 20230418&quot;},
{
  &quot;name&quot; : &quot;AH - 安南醫院&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;AHGL - 安南圖書區&quot;}]
},
{ &quot;name&quot; : &quot;av - av&quot;},
{ &quot;name&quot; : &quot;B007 - B007&quot;},
{ &quot;name&quot; : &quot;BCSB4 - BCSB4&quot;},
{ &quot;name&quot; : &quot;BX - 取書櫃1&quot;},
{ &quot;name&quot; : &quot;BY - 取書櫃2&quot;},
{
  &quot;name&quot; : &quot;CB - 北港分館&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;BAVN - 北港分館視聽區(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;BCIR - 北港分館流通櫃檯&quot;},
{ &quot;name&quot; : &quot;BCRA - 北港分館指參(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;BPAV - 北港分館視聽區&quot;},
{ &quot;name&quot; : &quot;BPCL - 北港分館書庫&quot;}]
},
{ &quot;name&quot; : &quot;cbook - cbook&quot;},
{ &quot;name&quot; : &quot;circd - circd&quot;},
{ &quot;name&quot; : &quot;clp - clp&quot;},
{
  &quot;name&quot; : &quot;CM - 北港附設醫院&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;BMHL - 北港附設醫院圖書室&quot;}]
},
{ &quot;name&quot; : &quot;CMUL - 中國醫藥大學圖書館 - CMUL - 中國醫藥大學圖書館&quot;},
{
  &quot;name&quot; : &quot;CU - 台中總館&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;CUAV - 台中總館視聽區(獨立專區)&quot;},
{ &quot;name&quot; : &quot;MAVN - 台中總館視聽區(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MAVR - 台中總館視聽區&quot;},
{ &quot;name&quot; : &quot;MCAT - 台中總館技服組&quot;},
{ &quot;name&quot; : &quot;MCBS - 台中總館密閉書庫&quot;},
{ &quot;name&quot; : &quot;MCIR - 台中總館流通櫃檯&quot;},
{ &quot;name&quot; : &quot;MCRA - 台中總館教師指定參考書(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MCSS - 台中總館B3裝訂期刊區&quot;},
{ &quot;name&quot; : &quot;MDIA - 台中總館博碩士論文區&quot;},
{ &quot;name&quot; : &quot;MEAS - 台中總館探索史懷哲之路專書區&quot;},
{ &quot;name&quot; : &quot;MEXM - 台中總館國考書區(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MFGA - 台中總館本校教職優良教材區&quot;},
{ &quot;name&quot; : &quot;MFPA - 台中總館本校教師升等資料區&quot;},
{ &quot;name&quot; : &quot;MFSA - 台中總館本校教職論著&quot;},
{ &quot;name&quot; : &quot;MHME - 台中總館人文專書區-醫學倫理(限館內閱覽)台中總館人文專書區-醫學倫理(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MHMH - 台中總館人文專書區-醫療史(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MHML - 台中總館人文專書區-醫學法律(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MHQC - 台中總館人文專書區-醫療品質(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MNBR - 台中總館新書展示區&quot;},
{ &quot;name&quot; : &quot;MPAA - 台中總館績效暨獲獎區(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MPBL - 台中總館PBL專書區(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MREF - 台中總館參考室&quot;},
{ &quot;name&quot; : &quot;MRSS - 台中總館閱覽組&quot;},
{ &quot;name&quot; : &quot;MS-C - 台中總館期刊複本櫃&quot;},
{ &quot;name&quot; : &quot;MSER - 台中總館期刊區&quot;},
{ &quot;name&quot; : &quot;MSPA - 台中總館研究計劃專書&quot;},
{ &quot;name&quot; : &quot;MSPB - 中醫醫史文獻室(限所內閱覽)&quot;},
{ &quot;name&quot; : &quot;MSPC - 台中總館特藏室&quot;},
{ &quot;name&quot; : &quot;MSTK - 台中總館書庫&quot;},
{ &quot;name&quot; : &quot;MYBK - 台中總館參考壁櫃&quot;},
{ &quot;name&quot; : &quot;new item 7 - new item 7&quot;},
{ &quot;name&quot; : &quot;ONLN - 台中總館線上資料&quot;}]
},
{ &quot;name&quot; : &quot;e-resources - 電子資源&quot;},
{ &quot;name&quot; : &quot;EB-P - EB-P&quot;},
{ &quot;name&quot; : &quot;elect - elect&quot;},
{ &quot;name&quot; : &quot;H-EQ - H-EQ&quot;},
{ &quot;name&quot; : &quot;H-MR - H-MR&quot;},
{ &quot;name&quot; : &quot;L - L&quot;},
{ &quot;name&quot; : &quot;L40 - L40&quot;},
{ &quot;name&quot; : &quot;LB 圖書總館 - LB 圖書總館&quot;},
{ &quot;name&quot; : &quot;LB-S - LB-S&quot;},
{
  &quot;name&quot; : &quot;LE - 語文教學中心&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;LEGL - 語文教學中心圖書室&quot;}]
},
{ &quot;name&quot; : &quot;LIB - LIB&quot;},
{
  &quot;name&quot; : &quot;new item 1 - new item 1&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;new item 3 - new item 3&quot;}]
},
{ &quot;name&quot; : &quot;new item 10 - new item 10&quot;},
{ &quot;name&quot; : &quot;new item 12 - new item 12&quot;},
{ &quot;name&quot; : &quot;new item 13 - new item 13&quot;},
{ &quot;name&quot; : &quot;new item 14 - new item 14&quot;},
{ &quot;name&quot; : &quot;new item 16 - new item 16&quot;},
{ &quot;name&quot; : &quot;new item 2 - new item 2&quot;},
{ &quot;name&quot; : &quot;new item 20 - new item 20&quot;},
{ &quot;name&quot; : &quot;new item 4 - new item 4&quot;},
{ &quot;name&quot; : &quot;new item 5 - new item 5&quot;},
{ &quot;name&quot; : &quot;new item 6 - 英才校區&quot;},
{ &quot;name&quot; : &quot;new item 8 - new item 8&quot;},
{ &quot;name&quot; : &quot;new item 9 - new item 9&quot;},
{ &quot;name&quot; : &quot;NPTU - NPTU&quot;},
{ &quot;name&quot; : &quot;OUK - OUK&quot;},
{
  &quot;name&quot; : &quot;PT - 培德醫院&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;new item 11 - new item 11&quot;},
{ &quot;name&quot; : &quot;new item 17 - new item 17&quot;},
{ &quot;name&quot; : &quot;PTGL - 培德醫院圖書區&quot;}]
},
{ &quot;name&quot; : &quot;ptext - ptext&quot;},
{ &quot;name&quot; : &quot;SB3 - SB3&quot;},
{ &quot;name&quot; : &quot;T-P - T-P&quot;},
{ &quot;name&quot; : &quot;TBBK - TBBK&quot;},
{
  &quot;name&quot; : &quot;TH - 台北分院&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;THGL - 台北分院圖書區&quot;},
{ &quot;name&quot; : &quot;THPA - 台北分院期刊區&quot;}]
},
{ &quot;name&quot; : &quot;W-P - W-P&quot;},
{
  &quot;name&quot; : &quot;YH - 豐原分院&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;new item 18 - new item 18&quot;},
{ &quot;name&quot; : &quot;new item 19 - new item 19&quot;},
{ &quot;name&quot; : &quot;YHGL - 豐原分院圖書區&quot;}]
},
{ &quot;name&quot; : &quot;z3llc - z3llc&quot;},
{ &quot;name&quot; : &quot;z6bkf - z6bkf&quot;},
{ &quot;name&quot; : &quot;zd1a2 - zd1a2&quot;},
{ &quot;name&quot; : &quot;zd1e - zd1e&quot;},
{ &quot;name&quot; : &quot;zdlf - zdlf&quot;},
{ &quot;name&quot; : &quot;五樓漫畫書專區 - 五樓漫畫書專區&quot;},
{ &quot;name&quot; : &quot;實體館藏 - 實體館藏&quot;},
{ &quot;name&quot; : &quot;綜合書庫 - 綜合書庫&quot;},
{ &quot;name&quot; : &quot;艾迪訊圖書館 - 艾迪訊圖書館&quot;},
{ &quot;name&quot; : &quot;附中出版物專區 - 附中出版物專區&quot;},
{ &quot;name&quot; : &quot;龍華科技大學圖書館 - 龍華科技大學圖書館&quot;}]
}]
}
		  {&quot;data&quot; :[{&quot;name&quot; : &quot;B可借圖書&quot;},{&quot;name&quot; : &quot;B電子資源&quot;},{&quot;name&quot; : &quot;eee&quot;},{&quot;name&quot; : &quot;M可借行動設備&quot;},{&quot;name&quot; : &quot;P可借期刊&quot;},{&quot;name&quot; : &quot;V可借視聽&quot;},{&quot;name&quot; : &quot;www&quot;},{&quot;name&quot; : &quot;不流通&quot;},{&quot;name&quot; : &quot;書箱借閱30天&quot;}]}


         

	
	
		

// 	dojo.event.topic.subscribe('info', closeDialogComponent('AssignedReports'));
	dojo.event.topic.subscribe('info',function(msg){closeDialogComponent('AssignedReports');});


  
 
  
    報表
  
  
 
 
  
 

	
	


	var refreshTimeout = null;
	function refresh() {
		clickDirectLink('refresher');
		setstytle();
	};
	function setstytle() {
		var ReportStatus = document.getElementById(&quot;ReportStatus&quot;);
		ReportStatus.style.width = &quot;663px&quot;;
		ReportStatus.style.height = &quot;360px&quot;;
	};



 
  
    報表狀態
  
  
 
 
  
 


	

	

			
		
	




		
				//alert(myMenu);
				//alert(JSON.stringify(cmThemePanel));
			//	cmRDraw ('myMenuID', myMenu, 'hbr', cmThemePanel, 'ThemePanel');
			 createMenuStr(myMenu);
		
		
			
	
		
        
          
    			
    				
    					
    						    
    						    
    						    	處理中...  
    						    
    												
    				
    			
    		
        
				



		
			
 
  
  
     
  
 
 
  
 


			   
 
  
  
     
  
 
 
  
 

   



			


		
		
        
        
        
		
				
			 Copyright© 2016 iNspire 4.4.0-SNAPSHOT by Claridy Solutions, Inc. All rights reserved.
		
		
		
        
        
		
		  
	function inputKeyCode(){ 
		keyCode=123; 
		keyEnable=true; 
	} 

		 		 

      

(function($){

	$(document).ready(function(){
		var curL = Date.parse(new Date()).valueOf();
		var activeL = Date.parse($('#activeTime').text()).valueOf();
		var inactiveL = activeL+300000;
		
		if( curL > activeL &amp;&amp; inactiveL > curL ) {
			$('.marquee').css('display','block');
		}else{
			$('.marquee').css('display','none');
		}
		
		$('.marquee').marquee({
			duration: 8000
		});
	})
	
	function marqueeInit(){
		console.log('關閉跑馬燈 &amp; 重新檢查');
		$('.marquee').css('display','none');
	}
	
})(jQuery);
 

	var keyCode;
	var keyEnable;
	document.onkeydown=KeyDown;
	function KeyDown(e){
		inputKeyCode();
		if(e.type ==&quot;keydown&quot;){
			var keyNum=window.event ? e.keyCode :e.which; 
		}else{
			var keyNum = e;
		}
			
		if(keyCode!=null &amp;&amp; keyEnable){
			if(keyNum==keyCode){
				createPopEdit(&quot;/inspireapp/UtilizatorPhraseDetails,$PopupBorder.$DirectLink_2.sdirect?updateParts=CloseReminderDialog&quot;);
			}	
		}
	}

	&lt;!--
tapestry.addOnLoad(function(e) {
dojo.require(&quot;tapestry.form&quot;);tapestry.form.registerForm(&quot;LanguageForm&quot;);

tapestry.form.registerForm(&quot;searchForm&quot;);
var field1 = new Ajax.Autocompleter(&quot;field1&quot;, &quot;field1choices&quot;, &quot;/inspireapp/Blacklist,searchComponent.field1.sdirect?sp=Sfield1&amp;sp=Snavigare24.valoare_afisare&amp;sp=Sstarts+with&amp;sp=1&amp;updateParts=field1&quot;, {&quot;method&quot;:&quot;get&quot;,&quot;frequency&quot;:0.2,&quot;minChars&quot;:1,&quot;onFailure&quot;:tapestry.error,&quot;encoding&quot;:&quot;UTF-8&quot;});
var field2 = new Ajax.Autocompleter(&quot;field2&quot;, &quot;field2choices&quot;, &quot;/inspireapp/Blacklist,searchComponent.field2.sdirect?sp=Sfield2&amp;sp=Scards.card_number&amp;sp=Sstarts+with&amp;sp=2&amp;updateParts=field2&quot;, {&quot;method&quot;:&quot;get&quot;,&quot;frequency&quot;:0.2,&quot;minChars&quot;:1,&quot;onFailure&quot;:tapestry.error,&quot;encoding&quot;:&quot;UTF-8&quot;});
tapestry.cleanConnect(&quot;formSubmitSearch&quot;, &quot;onclick&quot;, &quot;event1167465096&quot;);
        tapestry.event1167465096=function(e){
            tapestry.event.stopEvent(e);
            
                
                    tapestry.form.submit(&quot;searchForm&quot;, &quot;formSubmitSearch&quot;, {&quot;async&quot;:true,&quot;json&quot;:false,&quot;url&quot;:&quot;/inspireapp/Blacklist,searchComponent.searchForm.sdirect?updateParts=results&amp;updateParts=nrResults&amp;updateParts=autoRefreshZone&amp;updateParts=zSources&amp;updateParts=localSources&amp;updateParts=showOrderBy&quot;});
                
                
            
            
        };
        tapestry.connect(&quot;formSubmitSearch&quot;, &quot;onclick&quot;, &quot;event1167465096&quot;);
tapestry.cleanConnect(&quot;browse&quot;, &quot;onclick&quot;, &quot;event1984482014&quot;);
        tapestry.event1984482014=function(e){
            tapestry.event.stopEvent(e);
            
                
                    tapestry.form.submit(&quot;searchForm&quot;, &quot;browse&quot;, {&quot;async&quot;:true,&quot;json&quot;:false,&quot;url&quot;:&quot;/inspireapp/Blacklist,searchComponent.searchForm.sdirect?updateParts=results&amp;updateParts=nrResults&amp;updateParts=autoRefreshZone&amp;updateParts=zSources&amp;updateParts=localSources&quot;});
                
                
            
            
        };
        tapestry.connect(&quot;browse&quot;, &quot;onclick&quot;, &quot;event1984482014&quot;);
tapestry.cleanConnect(&quot;Submit&quot;, &quot;onclick&quot;, &quot;event178957379&quot;);
        tapestry.event178957379=function(e){
            tapestry.event.stopEvent(e);
            
                
                    tapestry.form.submit(&quot;searchForm&quot;, &quot;Submit&quot;, {&quot;async&quot;:true,&quot;json&quot;:false,&quot;url&quot;:&quot;/inspireapp/Blacklist,searchComponent.searchForm.sdirect?updateParts=searchComponent&quot;});
                
                
            
            
        };
        tapestry.connect(&quot;Submit&quot;, &quot;onclick&quot;, &quot;event178957379&quot;);
tapestry.cleanConnect(&quot;Submit_0&quot;, &quot;onclick&quot;, &quot;event1707222604&quot;);
        tapestry.event1707222604=function(e){
            tapestry.event.stopEvent(e);
            
                
                    tapestry.form.submit(&quot;searchForm&quot;, &quot;Submit_0&quot;, {&quot;async&quot;:true,&quot;json&quot;:false,&quot;url&quot;:&quot;/inspireapp/Blacklist,searchComponent.searchForm.sdirect?updateParts=searchComponent&quot;});
                
                
            
            
        };
        tapestry.connect(&quot;Submit_0&quot;, &quot;onclick&quot;, &quot;event1707222604&quot;);
tapestry.cleanConnect(&quot;Submit_1&quot;, &quot;onclick&quot;, &quot;event1707223370&quot;);
        tapestry.event1707223370=function(e){
            tapestry.event.stopEvent(e);
            
                
                    tapestry.form.submit(&quot;searchForm&quot;, &quot;Submit_1&quot;, {&quot;async&quot;:true,&quot;json&quot;:false,&quot;url&quot;:&quot;/inspireapp/Blacklist,searchComponent.searchForm.sdirect?updateParts=searchComponent&quot;});
                
                
            
            
        };
        tapestry.connect(&quot;Submit_1&quot;, &quot;onclick&quot;, &quot;event1707223370&quot;);
tapestry.cleanConnect(&quot;Submit_2&quot;, &quot;onclick&quot;, &quot;event1707223464&quot;);
        tapestry.event1707223464=function(e){
            tapestry.event.stopEvent(e);
            
                
                    tapestry.form.submit(&quot;searchForm&quot;, &quot;Submit_2&quot;, {&quot;async&quot;:true,&quot;json&quot;:false,&quot;url&quot;:&quot;/inspireapp/Blacklist,searchComponent.searchForm.sdirect?updateParts=searchComponent&quot;});
                
                
            
            
        };
        tapestry.connect(&quot;Submit_2&quot;, &quot;onclick&quot;, &quot;event1707223464&quot;);

tapestry.cleanConnect(&quot;sCriteria1&quot;, &quot;onchange&quot;, &quot;formEvent892686733&quot;);
                tapestry.formEvent892686733=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;Blacklist/searchComponent.sCriteria1&quot;, bcomponentid:&quot;sCriteria1&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;sCriteria1&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;sCriteria1&quot;, &quot;onchange&quot;, &quot;formEvent892686733&quot;);
tapestry.cleanConnect(&quot;comparator1&quot;, &quot;onchange&quot;, &quot;formEvent55030807&quot;);
                tapestry.formEvent55030807=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;Blacklist/searchComponent.comparator1&quot;, bcomponentid:&quot;comparator1&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;comparator1&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;comparator1&quot;, &quot;onchange&quot;, &quot;formEvent55030807&quot;);
tapestry.cleanConnect(&quot;sCriteria2&quot;, &quot;onchange&quot;, &quot;formEvent1403150071&quot;);
                tapestry.formEvent1403150071=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;Blacklist/searchComponent.sCriteria2&quot;, bcomponentid:&quot;sCriteria2&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;sCriteria2&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;sCriteria2&quot;, &quot;onchange&quot;, &quot;formEvent1403150071&quot;);
tapestry.cleanConnect(&quot;comparator2&quot;, &quot;onchange&quot;, &quot;formEvent255400393&quot;);
                tapestry.formEvent255400393=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;Blacklist/searchComponent.comparator2&quot;, bcomponentid:&quot;comparator2&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;comparator2&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;comparator2&quot;, &quot;onchange&quot;, &quot;formEvent255400393&quot;);
tapestry.cleanConnect(&quot;browseCriteria&quot;, &quot;onchange&quot;, &quot;formEvent151438188&quot;);
                tapestry.formEvent151438188=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;Blacklist/searchComponent.browseCriteria&quot;, bcomponentid:&quot;browseCriteria&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;browseCriteria&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;browseCriteria&quot;, &quot;onchange&quot;, &quot;formEvent151438188&quot;);
closeDialogComponent('TinreadDialog');

try {
  attachFocus('field1');
 }
 catch(e) {}
closeDialogComponent('AssignedReports');
closeDialogComponent('ReportStatus');
if(tapestry.fx) {
			var isResponse;
			tapestry.fx.attachAjaxStatus(function statusListener(status){
            	var fullDiv = document.getElementById('ajaxLoaderDiv');
            	var processDiv = document.getElementById('pressingDiv');
            	isResponse = status;
            	if(isResponse == false){
            		fullDiv.style.display=&quot;none&quot;;
            		processDiv.style.display=&quot;none&quot;;
            	}else{
            		fullDiv.style.display=&quot;&quot;;
            		setTimeout(
						function showProcessDiv(){
							if(isResponse==true){
								var processDiv = document.getElementById('pressingDiv');
								processDiv.style.display=&quot;&quot;;
							}
						}, 
						2000
					);
            	}
            }); 
		}
closeDialogComponent('TinreadErrorDialog');
closeDialogComponent('TinreadMessageDialog');});
// -->


id(&quot;orderby&quot;)</value>
      <webElementGuid>15e950e1-1ed8-4dd2-8d88-c0e05f9ed7b5</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;Body&quot;)</value>
      <webElementGuid>e91e954a-f5a0-4131-9467-61d85299c9b8</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//body[@id='Body']</value>
      <webElementGuid>8e58e070-0c2f-4aa8-bcb1-7bd8b25ee5a0</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>f1b40891-e6e8-40f4-94c4-3f651139dc56</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//body[@id = 'Body' and (text() = concat(&quot;

dojo.registerModulePath(&quot;tapestry&quot;, &quot;/inspireapp/assets/static/tapestry-4.1.6&quot;);



dojo.require(&quot;tapestry.namespace&quot;);
tapestry.requestEncoding=&quot; , &quot;'&quot; , &quot;UTF-8&quot; , &quot;'&quot; , &quot;;















































&lt;!--
// directory of where all the images are
var cmThemePanelBase = &quot; , &quot;'&quot; , &quot;/inspireapp/ThemePanel/&quot; , &quot;'&quot; , &quot;;

var cmThemePanel =
{

  	// main menu display attributes
  	//
  	// Note.  When the menu bar is horizontal,
  	// mainFolderLeft and mainFolderRight are
  	// put in &lt;span>&lt;/span>.  When the menu
  	// bar is vertical, they would be put in
  	// a separate TD cell.

  	// HTML code to the left of the folder item
  	mainFolderLeft: &quot; , &quot;'&quot; , &quot;&lt;img alt=&quot;&quot; src=&quot;&quot; , &quot;'&quot; , &quot; + cmThemePanelBase + &quot; , &quot;'&quot; , &quot;blank.gif&quot;/>&quot; , &quot;'&quot; , &quot;,
  	// HTML code to the right of the folder item
  	mainFolderRight: &quot; , &quot;'&quot; , &quot;&lt;img alt=&quot;&quot; src=&quot;&quot; , &quot;'&quot; , &quot; + cmThemePanelBase + &quot; , &quot;'&quot; , &quot;arrowdown.gif&quot;/>&quot; , &quot;'&quot; , &quot;,
	// HTML code to the left of the regular item
	mainItemLeft: &quot; , &quot;'&quot; , &quot;&lt;img alt=&quot;&quot; src=&quot;&quot; , &quot;'&quot; , &quot; + cmThemePanelBase + &quot; , &quot;'&quot; , &quot;blank.gif&quot;/>&quot; , &quot;'&quot; , &quot;,
	// HTML code to the right of the regular item
	mainItemRight: &quot; , &quot;'&quot; , &quot;&lt;img alt=&quot;&quot; src=&quot;&quot; , &quot;'&quot; , &quot; + cmThemePanelBase + &quot; , &quot;'&quot; , &quot;blank.gif&quot;/>&quot; , &quot;'&quot; , &quot;,

	// sub menu display attributes

	// HTML code to the left of the folder item
	folderLeft: &quot; , &quot;'&quot; , &quot;&lt;img alt=&quot;&quot; src=&quot;&quot; , &quot;'&quot; , &quot; + cmThemePanelBase + &quot; , &quot;'&quot; , &quot;blank.gif&quot;/>&quot; , &quot;'&quot; , &quot;,
	// HTML code to the right of the folder item
	folderRight: &quot; , &quot;'&quot; , &quot;&lt;img alt=&quot;&quot; src=&quot;&quot; , &quot;'&quot; , &quot; + cmThemePanelBase + &quot; , &quot;'&quot; , &quot;arrow.gif&quot;/>&quot; , &quot;'&quot; , &quot;,
	// HTML code to the left of the regular item
	itemLeft: &quot; , &quot;'&quot; , &quot;&lt;img alt=&quot;&quot; src=&quot;&quot; , &quot;'&quot; , &quot; + cmThemePanelBase + &quot; , &quot;'&quot; , &quot;blank.gif&quot;/>&quot; , &quot;'&quot; , &quot;,
	// HTML code to the right of the regular item
	itemRight: &quot; , &quot;'&quot; , &quot;&lt;img alt=&quot;&quot; src=&quot;&quot; , &quot;'&quot; , &quot; + cmThemePanelBase + &quot; , &quot;'&quot; , &quot;blank.gif&quot;/>&quot; , &quot;'&quot; , &quot;,
	// cell spacing for main menu
	mainSpacing: 0,
	// cell spacing for sub menus
	subSpacing: 0,
	// auto dispear time for submenus in milli-seconds
	delay: 500
};

// for sub menu horizontal split
var cmThemePanelHSplit = [_cmNoClick, &quot; , &quot;'&quot; , &quot;&lt;td colspan=&quot;3&quot; style=&quot;height: 5px; overflow: hidden&quot;>&lt;div class=&quot;ThemePanelMenuSplit&quot;>&lt;/div>&lt;/td>&quot; , &quot;'&quot; , &quot;];
// for vertical main menu horizontal split
var cmThemePanelMainHSplit = [_cmNoClick, &quot; , &quot;'&quot; , &quot;&lt;td colspan=&quot;3&quot; style=&quot;height: 5px; overflow: hidden&quot;>&lt;div class=&quot;ThemePanelMenuSplit&quot;>&lt;/div>&lt;/td>&quot; , &quot;'&quot; , &quot;];
// for horizontal main menu vertical split
var cmThemePanelMainVSplit = [_cmNoClick, &quot; , &quot;'&quot; , &quot;|&quot; , &quot;'&quot; , &quot;];






function restore(idx) {
		var element = document.getElementById(&quot;restore&quot;);
		var href = element.href+&quot;&amp;sp=l&quot;+idx;
		tapestry.linkOnClick(href,&quot; , &quot;'&quot; , &quot;restore&quot; , &quot;'&quot; , &quot;, false);
    }  
	var clearUrl=null;
    function deleteBibliographic(idx, warning) {
		var element = document.getElementById(&quot;deleteBibliographic&quot;);
		if (clearUrl == null) {
			clearUrl= element.href;
		}
		element.href = clearUrl+&quot;&amp;sp=l&quot;+idx+&quot;&amp;sp=&quot;+warning;
		clickDirectLink(&quot;deleteBibliographic&quot;);
    }
dojo.require(&quot;tapestry.event&quot;);




function openDialogComponent(compId, hideCloseIcon, hideMaximizeIcon, closeHiddenForm) {
  	if(closeHiddenForm!=null &amp;&amp; !String(closeHiddenForm)==&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;){
	    if(compId) {
	      var d = new MyAppAlert(compId, hideCloseIcon, hideMaximizeIcon, &quot; , &quot;'&quot; , &quot;/inspireapp/images/&quot; , &quot;'&quot; , &quot;, closeHiddenForm);
	    }
    }else{
    	if(compId) {
	      var d = new MyAppAlert(compId, hideCloseIcon, hideMaximizeIcon, &quot; , &quot;'&quot; , &quot;/inspireapp/images/&quot; , &quot;'&quot; , &quot;);
	    }
    }
  }
  function closeDialogComponent(compId) {
    if(compId) {
        MyAppAlert.closeDialog(compId);
    }
  }
function printItems(href) {
	hrefparameters = addCheck();
	if (hrefparameters!=null){

		popupwindow = window.open(&quot;&quot; ,&quot;printRecord&quot;, &quot;status=no,toolbar=0,scrollbars=yes,menubar=0,titlebar=0,resizable=0,width=1500,height=1500&quot;);
		popupwindow.moveTo(screen.width/2-325, screen.height/2-235);
		popupwindow.focus();
		popupwindow.location = href+hrefparameters;
	}
	else{
		var buttonObject = window.parent.document.getElementById(&quot;InfoMsg&quot;);
	  	buttonObject.onclick();
	}
	return false;
}

function addCheck(){
	var formObject = document.getElementById(&quot;searchForm&quot;);
	total=0;
	href=&quot;&quot;;
	for(var i=0;i &lt; formObject.length;i++) {
		var item = formObject.elements[i];
		if (item.name.indexOf(&quot;selectat&quot;) == 0) {
			if (item.checked == true) {
					id = formObject.elements[i-1];
					if (total == 0) delimitator = &quot;?&quot;;
					else delimitator = &quot;&amp;&quot;;
					href = href+delimitator+&quot;sp=&quot;+id.value;
					total++;					
			}
		}
	}
	if (total>0) return href; 
	else return null;
}

 
 
 function attachFocus(fieldName) {
    var field = document.getElementById(fieldName);
    field.focus();
    VirtualKeyboard.attachInput(field);
}
  
 
  imagesPath = &quot; , &quot;'&quot; , &quot;/inspireapp/images/&quot; , &quot;'&quot; , &quot;;
  
   function clickSearchButton() {
    var searchDiv = document.getElementById(&quot;HiddenSearch&quot;);
    var browseDiv = document.getElementById(&quot;HiddenBrowse&quot;);
    if (searchDiv) {
     if (browseDiv) {
	    if (searchDiv.style.display == &quot;none&quot;) {
	      var browseButton = document.getElementById(&quot;browse&quot;);
	      browseButton.click();
	    }
	    else {
		    var searchButton = document.getElementById(&quot;formSubmitSearch&quot;);
		    searchButton.click();
	    }
     }
    }
    else {
     if (browseDiv) {
          var browseButton = document.getElementById(&quot;browse&quot;);
	      browseButton.click();
     }
     else {
		    var searchButton = document.getElementById(&quot;formSubmitSearch&quot;);
		    searchButton.click();
     } 
    }
  }


function changeScheduledStatusReport(){
	var checkBox = document.getElementById(&quot;scheduledReport&quot;);
	var scheduledData = document.getElementById(&quot;ScheduledData&quot;);
	var sheduleImg = document.getElementById(&quot;sheduleImg&quot;);
	
	if (checkBox.checked==false) {
	    checkBox.checked = true;
	    scheduledData.style.display =&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	    sheduleImg.src =  &quot;images/arrow_checked.gif&quot;;
	  }
	else {
	 checkBox.checked = false;
	 scheduledData.style.display =&quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;
	 sheduleImg.src =  &quot;images/arrow_unchecked.gif&quot;;
   }
}
dojo.require(&quot;tapestry.fx&quot;);
// -->


	
	
	







		
	
	
		
			
				
				Hi, catc (CU)
				
				最近登入:2024-03-22 15:26:59 ,成功
				
				
					
					
                       0 筆新訊息
                    
                
				
				繁體中文English繁體中文
				
English
繁體中文
						
						
				
				
					
					登 出
				
			
		
		
		
			
			
				神資圖書館(15trunk機)
			
			
		
	
	
	
	
	
		
		
		
			
			
			 查詢  編目  流通  採購   期刊控制   場地設備  說明  我的帳戶  學科服務  管理  統計  清單  Portal  臉辨報表  ERM 
			簡單查詢進階查詢FRBR查詢最近的查詢已儲存的記錄FRBR作品書目書目記錄維護註記刪除之書目記錄書目記錄全域修改權威記錄權威記錄維護註記刪除之權威記錄權威記錄全域修改條碼號館藏資料維護註記刪除之館藏資料維護出版者出版者資料維護內部移送作業書單維護FRBR作品辦證讀者記錄維護檢查讀者註記刪除之讀者記錄讀者證卡記錄維護離職離校借閱檯讀者流通檯還書箱讀者查詢交易紀錄違規讀者查詢讀者個人違規記錄指定參考書交易歷史查詢一般搜尋交易館藏預約順位調整題名預約順位調整批次修改到期日內部移送作業盤點盤點維護跨館預約/調閱跨館預約/調閱需求移轉-寄送項目移轉-收到項目盤點比對批次報廢選擇報廢執行報廢已報廢歷史查詢書箱作業條碼號空號清單(條碼機印製)訂購檢查薦購請購訂購明細訂單登收用卷期登收程序發票款項維護預算經費供應商標案內部移送作業催缺候選催缺項目催缺記錄期刊主檔訂購檢查薦購請購核可訂購明細訂單登收用卷期登收程序發票款項維護預算經費供應商裝訂裝訂主檔維護擬裝設定送裝確認裝訂登收標案內部移送作業催缺候選催缺項目催缺記錄流通記錄預約記錄借用記錄維護分類維護單項維護參數設定流通政策維護預約審核場地設備黑名單書籤訊息排程工作日誌學科館藏館藏資料類型設定學科館藏維護學科預約教師資料維護讀者評價選單學科預約維護學科預約查詢教師評語學科報表學科館藏冊數統計教師學科服務時數與使用人次統計學科服務讀者清單學科報表學科館藏冊數統計教師學科服務時數與使用人次統計學科服務讀者清單一般設定類型/使用者(館員)片語設定系統設定貨幣匯率館藏地範本設定--依館員帳號翻譯轉換 選取清單格式屬性條碼號規範顯示設定圖書館系統屬性推薦範本設定館員新增申請QR Code 設定編目書目類型設定MARC規範格式設定顯示設定停用字SDI維護同義字條碼號原則對應流通流通通知單範本流通通知單參數設定流通通知單發送紀錄日常作業排程一般每周開閉館設定閉館日期維護交易紀錄類型交易方式可適用讀者身份交易方式可適用館藏流通類別交易方式可適用使用類別交易紀錄-館藏狀態流通政策讀者證卡原則單位所系夾書單欄位設定讀者閱讀興趣發送紀錄期刊週期傳閱清單維護 轉入Z39屬性設定更新書封上傳書封索引編修索引重建功能維護公佈欄維護意見回覆維護主題維護特別書籍維護心得分享東區互惠審核東區互惠匯出系統代碼維護貨幣代碼作者號維護科特號維護OPACSDI維護讀者評論管理Help維護熱門關鍵字過濾館員Log記錄館員登入/出記錄館員資料變更記錄讀者資料變更記錄權限設定MARC權限設定功能權限設定角色設定系統狀態排程管理排程監控者排程日誌查詢統計檢索點/檢索語統計報表不同時段使用次數統計書目下載次數統計電子資源統計電子資源-館藏點閱次數統計電子資源-讀者點閱次數統計電子資源-單位系所點閱次數統計編目統計館藏量統計報表類號區間統計報表圖書增減統計報表編目/館別/大類交叉分析報表大類/冊數統計表大類/特藏交叉統計表特藏/冊數統計表新書特藏大類交叉統計圖書增減表／月份／大類大類語言種數冊數統計大類/條碼號類別冊數統計圖書冊數分類統計單圖書冊數金額分類統計非書資料數量金額統計資料類型範圍館藏統計表編目工作量統計(個人)編目工作量統計(各館)館藏狀態統計報表流通統計讀者借閱排行榜書展借閱清單流通人數統計(學制分頁 符合科系)讀者類型借閱暨歸還流通量統計報表流通人次冊數統計  利用概況統計(日報表)讀者借閱交叉分析各館特藏預約統計報表館內使用統計讀者身份借閱統計讀者個人及單位所系借閱排行榜館藏資料借閱統計單位系所借閱排行榜單位系所借閱統計熱門借閱排行榜熱門預約排行榜讀者辦証暨補發人次統計表借閱交叉分析統計資料類型流通統計館員流通量統計單位(個人)保管圖書(非書)數量明細館際互借代還統計-本館館藏館際互借代還統計-他館館藏指定參考書借閱量統計表各大類熱門借閱排行榜大專院校圖書館讀者身份借閱清單及統計報表館藏流通量統計跨館預約/調閱館藏借閱排行榜罰款收款人數/次數統計跨館預約/調閱使用量統計罰款收款統計特定館藏借閱統計採購統計書商採購統計年度經費採購統計特藏訂購方式金額統計書商到書率書商績效統計期刊統計期刊到刊率統計報表平均收刊日統計報表歷年期刊借閱統計書商到書率書商催缺次數書商累計訂購次數期刊訂費漲幅統計逾期違約金統計期刊經費訂購統計報表場地設備場地設備使用統計表報表模組範本維護編目清單書目標籤報表Tag856檢查報表新版書標維護次分類交叉分析登錄簿條碼產生清單登錄簿清單條碼號空號清單單位科系館藏設定清單館藏狀態預約清單特殊館藏清單館藏狀態清單報表流通清單罰款未收款報表預約待取清單借閱(逾期)清單還書清單新書清單讀者欠款清單借閱逾期清單罰款收款清單預約撤架清單長期借閱圖書清單離線流通清單預約額滿資料清單單位系所借閱逾期清單跨館還書清單離職離校清單跨館移送清單預約保留結束清單指定參考書借閱量明細表未外借館藏清單重複讀者比對報表借閱逾期通知單取書櫃上架清單預約圖書清單採購清單年度經費採買書籍運用率訂購資料明細表擬購資料明細表請購單詢價單訂購逾期清單謝函書的催缺報表薦購書刊複本比對報表採購參考清單財產增加清冊財產減少清冊已付款未到清單訂購明細期刊清單期刊合訂本登錄簿交贈期刊清單期刊架位清單裝訂歷史清單薦購書刊複本比對報表期刊登錄簿索贈清單期刊查驗清單已付款未到清單場地設備預約取消清單臉辨用戶使用分析表臉辨用戶與圖書分類關係表期間辦證統計_年報期間辦證統計_月報期間辦證明細期間辦證異動統計_年報期間辦證異動統計_月報辦證異動明細臉辨使用紀錄統計_年報臉辨使用紀錄統計_月報臉辨使用紀錄明細人數統計表人數統計明細表人次統計表人次統計明細表熱門時段統計表讀者進出排行榜期間辦證刪除明細系統參數設定IP區間管理延伸查詢設定系統設定檔資源瀏覽/後分類設定資源到期通知設定可使用身分類別設定 代碼管理所屬資料庫清單共用代碼檔共用代碼類別電子資源管理資源前端顯示設定Ezproxy設定檔維護電子資料庫/網路資源電子期刊/電子書電子資源批次匯入電子資源副檔匯入讀者回報資源連線異常 Meniuri.erm.9000報表點閱資源排行榜依學院別使用統計依單位別使用統計依月分資源使用統計依學院登入統計表依系所登入統計表依身分登入統計表登入清單依身分別使用統計表使用者使用資源清單 收藏資源排行榜偵測電子資源狀態設定偵測資源時間設定偵測電子資源查詢電子資源狀態電子資源狀況報表
				
	


		
			
				場地設備 > 場地設備黑名單		
			
	


							
	jQuery(function () {
		jQuery(&quot;div[id=&quot; , &quot;'&quot; , &quot;hiddenDiacritics&quot; , &quot;'&quot; , &quot;]&quot;).draggable({
	        containment: &quot;#box&quot;,
	        scroll: false
	});
	});
	jQuery(function () {
		jQuery(&quot;div[id=&quot; , &quot;'&quot; , &quot;symb_div&quot; , &quot;'&quot; , &quot;]&quot;).draggable({
			containment: &quot;#box&quot;,
			scroll: false
	});
	});


function checkElement(elementId, checkedElem){
	href = document.getElementById(&quot;checkInform&quot;).href+&quot; , &quot;'&quot; , &quot;&amp;sp=l&quot; , &quot;'&quot; , &quot;+elementId+&quot; , &quot;'&quot; , &quot;&amp;sp=&quot; , &quot;'&quot; , &quot;+(checkedElem==true?&quot; , &quot;'&quot; , &quot;T&quot; , &quot;'&quot; , &quot;:&quot; , &quot;'&quot; , &quot;F&quot; , &quot;'&quot; , &quot;);
	return tapestry.linkOnClick(href,&quot; , &quot;'&quot; , &quot;checkInform&quot; , &quot;'&quot; , &quot;, false);
}

function syncWithOrder(){
document.getElementById(&quot;orderCriteria&quot;).value = document.getElementById(&quot;browseCriteria&quot;).value;
}
function syncWithBrowse(){
document.getElementById(&quot;browseCriteria&quot;).value = document.getElementById(&quot;orderCriteria&quot;).value;
}

var refreshTime = 0;
function apasa() {
  refreshTime = 2500;

  clickLinkSubmit(&quot;searchForm&quot;, &quot;scriptSubmit&quot;);
}

function showingOrderBy(status){
	document.getElementById(&quot;showOrderBy&quot;).style.display = status;
}
function remakeSearch(){
	if(typeof(tapestry.form)!=&quot;undefined&quot;) clickLinkSubmit(&quot; , &quot;'&quot; , &quot;searchForm&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;remakeSearch&quot; , &quot;'&quot; , &quot;);
	else window.setTimeout(&quot;remakeSearch()&quot;,500);
}

function remakePaginatedSearch(){
	clickLinkSubmit(&quot; , &quot;'&quot; , &quot;searchForm&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;remakePaginatedSearch&quot; , &quot;'&quot; , &quot;);
}
j(document).ready(function(){
	j(&quot;#resetbutton&quot;).click(function(){	
		//j(&quot;#reseter&quot;).click();
		var h=j(&quot;#reseter&quot;).attr(&quot;href&quot;);
		window.location=h;
	});
	
    // JSON 資料結構模擬  start
    try{
    var treeJsonData = JSON.parse(j(&quot; , &quot;'&quot; , &quot;#locData&quot; , &quot;'&quot; , &quot;).text());
    
    j(&quot; , &quot;'&quot; , &quot;.place&quot; , &quot;'&quot; , &quot;).treeoptions({
        data: treeJsonData,
        openImg: &quot; , &quot;'&quot; , &quot;/inspireapp/images/ico_add.gif&quot; , &quot;'&quot; , &quot;, // img 路徑
        cleanImg:&quot; , &quot;'&quot; , &quot;/inspireapp/images/clear.gif&quot; , &quot;'&quot; , &quot; // img 路徑
    });
    }catch(e){
    }
    try{
    var treeJsonData1 = JSON.parse(j(&quot; , &quot;'&quot; , &quot;#itemClassData&quot; , &quot;'&quot; , &quot;).text());
    
    j(&quot; , &quot;'&quot; , &quot;.place0&quot; , &quot;'&quot; , &quot;).treeoptions({
        data: treeJsonData1,
        openImg: &quot; , &quot;'&quot; , &quot;/inspireapp/images/ico_add.gif&quot; , &quot;'&quot; , &quot;, // img 路徑
        cleanImg:&quot; , &quot;'&quot; , &quot;/inspireapp/images/clear.gif&quot; , &quot;'&quot; , &quot; // img 路徑
    });
    }catch(e){
    }
    try{
    var treeJsonData2 = JSON.parse(j(&quot; , &quot;'&quot; , &quot;#materialTypeData&quot; , &quot;'&quot; , &quot;).text());
    
    j(&quot; , &quot;'&quot; , &quot;.place1&quot; , &quot;'&quot; , &quot;).treeoptions({
        data: treeJsonData2,
        openImg: &quot; , &quot;'&quot; , &quot;/inspireapp/images/ico_add.gif&quot; , &quot;'&quot; , &quot;, // img 路徑
        cleanImg:&quot; , &quot;'&quot; , &quot;/inspireapp/images/clear.gif&quot; , &quot;'&quot; , &quot; // img 路徑
    });
    }catch(e){
    }
    try{
        var treeJsonData3 = JSON.parse(j(&quot; , &quot;'&quot; , &quot;#languageData&quot; , &quot;'&quot; , &quot;).text());
        
        j(&quot; , &quot;'&quot; , &quot;.place2&quot; , &quot;'&quot; , &quot;).treeoptions({
            data: treeJsonData3,
            openImg: &quot; , &quot;'&quot; , &quot;/inspireapp/images/ico_add.gif&quot; , &quot;'&quot; , &quot;, // img 路徑
            cleanImg:&quot; , &quot;'&quot; , &quot;/inspireapp/images/clear.gif&quot; , &quot;'&quot; , &quot; // img 路徑
        });
    }catch(e){
    }
    try{
        var treeJsonData4 = JSON.parse(j(&quot; , &quot;'&quot; , &quot;#langCataData&quot; , &quot;'&quot; , &quot;).text());
        
        j(&quot; , &quot;'&quot; , &quot;.place3&quot; , &quot;'&quot; , &quot;).treeoptions({
            data: treeJsonData4,
            openImg: &quot; , &quot;'&quot; , &quot;/inspireapp/images/ico_add.gif&quot; , &quot;'&quot; , &quot;, // img 路徑
            cleanImg:&quot; , &quot;'&quot; , &quot;/inspireapp/images/clear.gif&quot; , &quot;'&quot; , &quot; // img 路徑
        });
    }catch(e){
    }
    
    try{
        var departmentJson = JSON.parse(j(&quot; , &quot;'&quot; , &quot;#departmentData&quot; , &quot;'&quot; , &quot;).text());
        
        j(&quot; , &quot;'&quot; , &quot;.multipleDepartmentClass&quot; , &quot;'&quot; , &quot;).treeoptions({
            data: departmentJson,
            openImg: &quot; , &quot;'&quot; , &quot;/inspireapp/images/ico_add.gif&quot; , &quot;'&quot; , &quot;,
            cleanImg:&quot; , &quot;'&quot; , &quot;/inspireapp/images/clear.gif&quot; , &quot;'&quot; , &quot;
        });
    }catch(e){}
});

function runScript(e) {
    if (e.keyCode == 13) {
        document.getElementById(&quot;browse&quot;).click();
        return false;
    }
}

function createPopEdit(href) {

	popupwindow = window.open(&quot;&quot; ,&quot;MeniuCatalogare&quot;, &quot;status=no,toolbar=0,scrollbars=yes,menubar=0,titlebar=0,resizable=1,width=1500,height=1500&quot;);
	popupwindow.moveTo(screen.width/2-435, screen.height/2-300);
	popupwindow.focus();

	popupwindow.location = href;

	if (popupwindow == null) popupwindow.opener = self;
	return false;

}

function createUploadPopEdit(href) {
	popupwindow = window.open(&quot;&quot; ,&quot;Upload&quot;, &quot;status=no,toolbar=0,scrollbars=yes,menubar=0,titlebar=0,resizable=1,top=&quot; + (screen.height-150)/2 + &quot;,left=&quot; + (screen.width-600)/2 + &quot;,width=600,height=250&quot;);
	popupwindow.focus();

	popupwindow.location = href;

	if (popupwindow == null) popupwindow.opener = self;
	return false;
};



	
	
		
			refreshTime = 0;
		
	



			 
				
						
						查詢
							
						
						
				   
				
						
							瀏覽
								
							
						
				
				
	  		
					
						
						 
							新增 
		
				
				
				
				
				
				
				
				
				
				















































	

	
	
	
	
		
			
			查詢條件
				
					
日期
讀者姓名
讀者證號

				
					
起始以
包含
等於(=)
大於(>=)
小於(&lt;=)
不等於(≠)

				
				
				
					
					
						
						
							
						
					
				
				
				
				
					
and
or
not

					
日期
讀者姓名
讀者證號

				
					
起始以
包含
等於(=)
大於(>=)
小於(&lt;=)
不等於(≠)

				
				
				
					
					
						
						
							
						
					
				
				
				
				
				
				
					
								
		                    
		                    	
								
				
			
			
				
				
				  

					
				

             
			
			
			
				
		
	

	
	
	
		瀏覽條件:
		
			
日期
讀者姓名
讀者證號

		
		
			
			
			
			起始以
			
			:
		
		
			
			
				 
				
				

//&lt;![CDATA[

					document.getElementById(&quot;listField&quot;).focus();
				
//]]&gt;


			
		
	    
		 
			
      
	
	 
	
	
    
		            
		        
		              

		          
			 

			
			
  					
					 
					     
					      
					       
					       
					       
					 		
						                  排序條件 :  
							  
							  
日期
讀者姓名
讀者證號

						       
						       
						      
						      
						   
							
								
升冪
降冪

							 
							
								
								
								



                            	
                              		
                            	
                            	
							
						
						

					 
					 
					   
         
			 
			 
			 
             
                
                    
                      
                        符號表:
                        
                      
                      
                        
貨幣：
￥   
￡   
₤
₣
₢      
₡
₠
₥
₦
₧ 
₨
₩
₪
₫
€

數學符號：
±
Ω
λ
β
α
θ
π
μ
≠
≤
≥
∑

日文：
あ
い
う
え
お
か
き
く
け
こ
さ
し

す
せ
そ
た
ち
つ
て
と
な
に
ぬ
ね

の
は
ひ
ふ
へ
ほ
ま
み
む
め
も
や

ゆ
よ
ら
り
る
れ
ろ
わ
を
ん

が
ぎ

ぐ
げ
ご
ざ
じ
ず
ぜ
ぞ
だ
ぢ
づ
で

ど
ば
び
ぶ
べ
ぼ

ぱ
ぴ
ぷ
ぺ
ぽ


ア
イ
ウ
エ
オ
カ
キ
ク
ケ
コ
サ
シ

ス
セ
ソ
タ
チ
ツ
テ
ト
ナ
ニ
ヌ
ネ

ノ
ハ
ヒ
フ
ヘ
ホ
マ
ミ
ム
メ
モ
ヤ

ユ
ヨ
ラ
リ
ル
レ
ロ
ワ
ヲ
ン
ガ
ギ

グ
ゲ
ゴ
ザ
ジ
ズ
ゼ
ゾ
ダ
ヂ
ヅ
デ

ド
バ
ビ
ブ
ベ
ボ
パ
ピ
プ
ペ
ポ

ぃ
ぅ
ぇ
ぉ
っ
ゃ
ゅ
ょ
ゎ
ァ
ィ
ゥ

ェ
ォ
ッ
ャ
ュ
ョ
ヮ
々

羅馬數字：
Ⅰ
Ⅱ
Ⅲ
Ⅳ
Ⅴ
Ⅵ
Ⅶ
Ⅷ
Ⅸ
Ⅹ


                      
                    
                
                
             
			 
			 
  

  
			
		
		
		 
10
20
50
100

      	 
      	 
		 
		
	
  
  	

	
     	 
		



	

	

		
	
		 


 
 
 
 



 
  
    Go To Page
  
  
 
 
  
 




		  
		    
		  
		  
		  



		  {
  &quot;data&quot; : [  
{
  &quot;name&quot; : &quot;CMUL - 神資圖書館&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;2 - 2&quot;},
{ &quot;name&quot; : &quot;123 - 123&quot;},
{ &quot;name&quot; : &quot;20230417 - 20230417&quot;},
{ &quot;name&quot; : &quot;20230418 - 20230418&quot;},
{
  &quot;name&quot; : &quot;AH - 安南醫院&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;AHGL - 安南圖書區&quot;}]
},
{ &quot;name&quot; : &quot;av - av&quot;},
{ &quot;name&quot; : &quot;B007 - B007&quot;},
{ &quot;name&quot; : &quot;BCSB4 - BCSB4&quot;},
{ &quot;name&quot; : &quot;BX - 取書櫃1&quot;},
{ &quot;name&quot; : &quot;BY - 取書櫃2&quot;},
{
  &quot;name&quot; : &quot;CB - 北港分館&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;BAVN - 北港分館視聽區(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;BCIR - 北港分館流通櫃檯&quot;},
{ &quot;name&quot; : &quot;BCRA - 北港分館指參(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;BPAV - 北港分館視聽區&quot;},
{ &quot;name&quot; : &quot;BPCL - 北港分館書庫&quot;}]
},
{ &quot;name&quot; : &quot;cbook - cbook&quot;},
{ &quot;name&quot; : &quot;circd - circd&quot;},
{ &quot;name&quot; : &quot;clp - clp&quot;},
{
  &quot;name&quot; : &quot;CM - 北港附設醫院&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;BMHL - 北港附設醫院圖書室&quot;}]
},
{ &quot;name&quot; : &quot;CMUL - 中國醫藥大學圖書館 - CMUL - 中國醫藥大學圖書館&quot;},
{
  &quot;name&quot; : &quot;CU - 台中總館&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;CUAV - 台中總館視聽區(獨立專區)&quot;},
{ &quot;name&quot; : &quot;MAVN - 台中總館視聽區(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MAVR - 台中總館視聽區&quot;},
{ &quot;name&quot; : &quot;MCAT - 台中總館技服組&quot;},
{ &quot;name&quot; : &quot;MCBS - 台中總館密閉書庫&quot;},
{ &quot;name&quot; : &quot;MCIR - 台中總館流通櫃檯&quot;},
{ &quot;name&quot; : &quot;MCRA - 台中總館教師指定參考書(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MCSS - 台中總館B3裝訂期刊區&quot;},
{ &quot;name&quot; : &quot;MDIA - 台中總館博碩士論文區&quot;},
{ &quot;name&quot; : &quot;MEAS - 台中總館探索史懷哲之路專書區&quot;},
{ &quot;name&quot; : &quot;MEXM - 台中總館國考書區(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MFGA - 台中總館本校教職優良教材區&quot;},
{ &quot;name&quot; : &quot;MFPA - 台中總館本校教師升等資料區&quot;},
{ &quot;name&quot; : &quot;MFSA - 台中總館本校教職論著&quot;},
{ &quot;name&quot; : &quot;MHME - 台中總館人文專書區-醫學倫理(限館內閱覽)台中總館人文專書區-醫學倫理(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MHMH - 台中總館人文專書區-醫療史(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MHML - 台中總館人文專書區-醫學法律(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MHQC - 台中總館人文專書區-醫療品質(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MNBR - 台中總館新書展示區&quot;},
{ &quot;name&quot; : &quot;MPAA - 台中總館績效暨獲獎區(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MPBL - 台中總館PBL專書區(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MREF - 台中總館參考室&quot;},
{ &quot;name&quot; : &quot;MRSS - 台中總館閱覽組&quot;},
{ &quot;name&quot; : &quot;MS-C - 台中總館期刊複本櫃&quot;},
{ &quot;name&quot; : &quot;MSER - 台中總館期刊區&quot;},
{ &quot;name&quot; : &quot;MSPA - 台中總館研究計劃專書&quot;},
{ &quot;name&quot; : &quot;MSPB - 中醫醫史文獻室(限所內閱覽)&quot;},
{ &quot;name&quot; : &quot;MSPC - 台中總館特藏室&quot;},
{ &quot;name&quot; : &quot;MSTK - 台中總館書庫&quot;},
{ &quot;name&quot; : &quot;MYBK - 台中總館參考壁櫃&quot;},
{ &quot;name&quot; : &quot;new item 7 - new item 7&quot;},
{ &quot;name&quot; : &quot;ONLN - 台中總館線上資料&quot;}]
},
{ &quot;name&quot; : &quot;e-resources - 電子資源&quot;},
{ &quot;name&quot; : &quot;EB-P - EB-P&quot;},
{ &quot;name&quot; : &quot;elect - elect&quot;},
{ &quot;name&quot; : &quot;H-EQ - H-EQ&quot;},
{ &quot;name&quot; : &quot;H-MR - H-MR&quot;},
{ &quot;name&quot; : &quot;L - L&quot;},
{ &quot;name&quot; : &quot;L40 - L40&quot;},
{ &quot;name&quot; : &quot;LB 圖書總館 - LB 圖書總館&quot;},
{ &quot;name&quot; : &quot;LB-S - LB-S&quot;},
{
  &quot;name&quot; : &quot;LE - 語文教學中心&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;LEGL - 語文教學中心圖書室&quot;}]
},
{ &quot;name&quot; : &quot;LIB - LIB&quot;},
{
  &quot;name&quot; : &quot;new item 1 - new item 1&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;new item 3 - new item 3&quot;}]
},
{ &quot;name&quot; : &quot;new item 10 - new item 10&quot;},
{ &quot;name&quot; : &quot;new item 12 - new item 12&quot;},
{ &quot;name&quot; : &quot;new item 13 - new item 13&quot;},
{ &quot;name&quot; : &quot;new item 14 - new item 14&quot;},
{ &quot;name&quot; : &quot;new item 16 - new item 16&quot;},
{ &quot;name&quot; : &quot;new item 2 - new item 2&quot;},
{ &quot;name&quot; : &quot;new item 20 - new item 20&quot;},
{ &quot;name&quot; : &quot;new item 4 - new item 4&quot;},
{ &quot;name&quot; : &quot;new item 5 - new item 5&quot;},
{ &quot;name&quot; : &quot;new item 6 - 英才校區&quot;},
{ &quot;name&quot; : &quot;new item 8 - new item 8&quot;},
{ &quot;name&quot; : &quot;new item 9 - new item 9&quot;},
{ &quot;name&quot; : &quot;NPTU - NPTU&quot;},
{ &quot;name&quot; : &quot;OUK - OUK&quot;},
{
  &quot;name&quot; : &quot;PT - 培德醫院&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;new item 11 - new item 11&quot;},
{ &quot;name&quot; : &quot;new item 17 - new item 17&quot;},
{ &quot;name&quot; : &quot;PTGL - 培德醫院圖書區&quot;}]
},
{ &quot;name&quot; : &quot;ptext - ptext&quot;},
{ &quot;name&quot; : &quot;SB3 - SB3&quot;},
{ &quot;name&quot; : &quot;T-P - T-P&quot;},
{ &quot;name&quot; : &quot;TBBK - TBBK&quot;},
{
  &quot;name&quot; : &quot;TH - 台北分院&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;THGL - 台北分院圖書區&quot;},
{ &quot;name&quot; : &quot;THPA - 台北分院期刊區&quot;}]
},
{ &quot;name&quot; : &quot;W-P - W-P&quot;},
{
  &quot;name&quot; : &quot;YH - 豐原分院&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;new item 18 - new item 18&quot;},
{ &quot;name&quot; : &quot;new item 19 - new item 19&quot;},
{ &quot;name&quot; : &quot;YHGL - 豐原分院圖書區&quot;}]
},
{ &quot;name&quot; : &quot;z3llc - z3llc&quot;},
{ &quot;name&quot; : &quot;z6bkf - z6bkf&quot;},
{ &quot;name&quot; : &quot;zd1a2 - zd1a2&quot;},
{ &quot;name&quot; : &quot;zd1e - zd1e&quot;},
{ &quot;name&quot; : &quot;zdlf - zdlf&quot;},
{ &quot;name&quot; : &quot;五樓漫畫書專區 - 五樓漫畫書專區&quot;},
{ &quot;name&quot; : &quot;實體館藏 - 實體館藏&quot;},
{ &quot;name&quot; : &quot;綜合書庫 - 綜合書庫&quot;},
{ &quot;name&quot; : &quot;艾迪訊圖書館 - 艾迪訊圖書館&quot;},
{ &quot;name&quot; : &quot;附中出版物專區 - 附中出版物專區&quot;},
{ &quot;name&quot; : &quot;龍華科技大學圖書館 - 龍華科技大學圖書館&quot;}]
}]
}
		  {&quot;data&quot; :[{&quot;name&quot; : &quot;B可借圖書&quot;},{&quot;name&quot; : &quot;B電子資源&quot;},{&quot;name&quot; : &quot;eee&quot;},{&quot;name&quot; : &quot;M可借行動設備&quot;},{&quot;name&quot; : &quot;P可借期刊&quot;},{&quot;name&quot; : &quot;V可借視聽&quot;},{&quot;name&quot; : &quot;www&quot;},{&quot;name&quot; : &quot;不流通&quot;},{&quot;name&quot; : &quot;書箱借閱30天&quot;}]}


         

	
	
		

// 	dojo.event.topic.subscribe(&quot; , &quot;'&quot; , &quot;info&quot; , &quot;'&quot; , &quot;, closeDialogComponent(&quot; , &quot;'&quot; , &quot;AssignedReports&quot; , &quot;'&quot; , &quot;));
	dojo.event.topic.subscribe(&quot; , &quot;'&quot; , &quot;info&quot; , &quot;'&quot; , &quot;,function(msg){closeDialogComponent(&quot; , &quot;'&quot; , &quot;AssignedReports&quot; , &quot;'&quot; , &quot;);});


  
 
  
    報表
  
  
 
 
  
 

	
	


	var refreshTimeout = null;
	function refresh() {
		clickDirectLink(&quot; , &quot;'&quot; , &quot;refresher&quot; , &quot;'&quot; , &quot;);
		setstytle();
	};
	function setstytle() {
		var ReportStatus = document.getElementById(&quot;ReportStatus&quot;);
		ReportStatus.style.width = &quot;663px&quot;;
		ReportStatus.style.height = &quot;360px&quot;;
	};



 
  
    報表狀態
  
  
 
 
  
 


	

	

			
		
	




		
				//alert(myMenu);
				//alert(JSON.stringify(cmThemePanel));
			//	cmRDraw (&quot; , &quot;'&quot; , &quot;myMenuID&quot; , &quot;'&quot; , &quot;, myMenu, &quot; , &quot;'&quot; , &quot;hbr&quot; , &quot;'&quot; , &quot;, cmThemePanel, &quot; , &quot;'&quot; , &quot;ThemePanel&quot; , &quot;'&quot; , &quot;);
			 createMenuStr(myMenu);
		
		
			
	
		
        
          
    			
    				
    					
    						    
    						    
    						    	處理中...  
    						    
    												
    				
    			
    		
        
				



		
			
 
  
  
     
  
 
 
  
 


			   
 
  
  
     
  
 
 
  
 

   



			


		
		
        
        
        
		
				
			 Copyright© 2016 iNspire 4.4.0-SNAPSHOT by Claridy Solutions, Inc. All rights reserved.
		
		
		
        
        
		
		  
	function inputKeyCode(){ 
		keyCode=123; 
		keyEnable=true; 
	} 

		 		 

      

(function($){

	$(document).ready(function(){
		var curL = Date.parse(new Date()).valueOf();
		var activeL = Date.parse($(&quot; , &quot;'&quot; , &quot;#activeTime&quot; , &quot;'&quot; , &quot;).text()).valueOf();
		var inactiveL = activeL+300000;
		
		if( curL > activeL &amp;&amp; inactiveL > curL ) {
			$(&quot; , &quot;'&quot; , &quot;.marquee&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;block&quot; , &quot;'&quot; , &quot;);
		}else{
			$(&quot; , &quot;'&quot; , &quot;.marquee&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;);
		}
		
		$(&quot; , &quot;'&quot; , &quot;.marquee&quot; , &quot;'&quot; , &quot;).marquee({
			duration: 8000
		});
	})
	
	function marqueeInit(){
		console.log(&quot; , &quot;'&quot; , &quot;關閉跑馬燈 &amp; 重新檢查&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.marquee&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;);
	}
	
})(jQuery);
 

	var keyCode;
	var keyEnable;
	document.onkeydown=KeyDown;
	function KeyDown(e){
		inputKeyCode();
		if(e.type ==&quot;keydown&quot;){
			var keyNum=window.event ? e.keyCode :e.which; 
		}else{
			var keyNum = e;
		}
			
		if(keyCode!=null &amp;&amp; keyEnable){
			if(keyNum==keyCode){
				createPopEdit(&quot;/inspireapp/UtilizatorPhraseDetails,$PopupBorder.$DirectLink_2.sdirect?updateParts=CloseReminderDialog&quot;);
			}	
		}
	}

	&lt;!--
tapestry.addOnLoad(function(e) {
dojo.require(&quot;tapestry.form&quot;);tapestry.form.registerForm(&quot;LanguageForm&quot;);

tapestry.form.registerForm(&quot;searchForm&quot;);
var field1 = new Ajax.Autocompleter(&quot;field1&quot;, &quot;field1choices&quot;, &quot;/inspireapp/Blacklist,searchComponent.field1.sdirect?sp=Sfield1&amp;sp=Snavigare24.valoare_afisare&amp;sp=Sstarts+with&amp;sp=1&amp;updateParts=field1&quot;, {&quot;method&quot;:&quot;get&quot;,&quot;frequency&quot;:0.2,&quot;minChars&quot;:1,&quot;onFailure&quot;:tapestry.error,&quot;encoding&quot;:&quot;UTF-8&quot;});
var field2 = new Ajax.Autocompleter(&quot;field2&quot;, &quot;field2choices&quot;, &quot;/inspireapp/Blacklist,searchComponent.field2.sdirect?sp=Sfield2&amp;sp=Scards.card_number&amp;sp=Sstarts+with&amp;sp=2&amp;updateParts=field2&quot;, {&quot;method&quot;:&quot;get&quot;,&quot;frequency&quot;:0.2,&quot;minChars&quot;:1,&quot;onFailure&quot;:tapestry.error,&quot;encoding&quot;:&quot;UTF-8&quot;});
tapestry.cleanConnect(&quot;formSubmitSearch&quot;, &quot;onclick&quot;, &quot;event1167465096&quot;);
        tapestry.event1167465096=function(e){
            tapestry.event.stopEvent(e);
            
                
                    tapestry.form.submit(&quot;searchForm&quot;, &quot;formSubmitSearch&quot;, {&quot;async&quot;:true,&quot;json&quot;:false,&quot;url&quot;:&quot;/inspireapp/Blacklist,searchComponent.searchForm.sdirect?updateParts=results&amp;updateParts=nrResults&amp;updateParts=autoRefreshZone&amp;updateParts=zSources&amp;updateParts=localSources&amp;updateParts=showOrderBy&quot;});
                
                
            
            
        };
        tapestry.connect(&quot;formSubmitSearch&quot;, &quot;onclick&quot;, &quot;event1167465096&quot;);
tapestry.cleanConnect(&quot;browse&quot;, &quot;onclick&quot;, &quot;event1984482014&quot;);
        tapestry.event1984482014=function(e){
            tapestry.event.stopEvent(e);
            
                
                    tapestry.form.submit(&quot;searchForm&quot;, &quot;browse&quot;, {&quot;async&quot;:true,&quot;json&quot;:false,&quot;url&quot;:&quot;/inspireapp/Blacklist,searchComponent.searchForm.sdirect?updateParts=results&amp;updateParts=nrResults&amp;updateParts=autoRefreshZone&amp;updateParts=zSources&amp;updateParts=localSources&quot;});
                
                
            
            
        };
        tapestry.connect(&quot;browse&quot;, &quot;onclick&quot;, &quot;event1984482014&quot;);
tapestry.cleanConnect(&quot;Submit&quot;, &quot;onclick&quot;, &quot;event178957379&quot;);
        tapestry.event178957379=function(e){
            tapestry.event.stopEvent(e);
            
                
                    tapestry.form.submit(&quot;searchForm&quot;, &quot;Submit&quot;, {&quot;async&quot;:true,&quot;json&quot;:false,&quot;url&quot;:&quot;/inspireapp/Blacklist,searchComponent.searchForm.sdirect?updateParts=searchComponent&quot;});
                
                
            
            
        };
        tapestry.connect(&quot;Submit&quot;, &quot;onclick&quot;, &quot;event178957379&quot;);
tapestry.cleanConnect(&quot;Submit_0&quot;, &quot;onclick&quot;, &quot;event1707222604&quot;);
        tapestry.event1707222604=function(e){
            tapestry.event.stopEvent(e);
            
                
                    tapestry.form.submit(&quot;searchForm&quot;, &quot;Submit_0&quot;, {&quot;async&quot;:true,&quot;json&quot;:false,&quot;url&quot;:&quot;/inspireapp/Blacklist,searchComponent.searchForm.sdirect?updateParts=searchComponent&quot;});
                
                
            
            
        };
        tapestry.connect(&quot;Submit_0&quot;, &quot;onclick&quot;, &quot;event1707222604&quot;);
tapestry.cleanConnect(&quot;Submit_1&quot;, &quot;onclick&quot;, &quot;event1707223370&quot;);
        tapestry.event1707223370=function(e){
            tapestry.event.stopEvent(e);
            
                
                    tapestry.form.submit(&quot;searchForm&quot;, &quot;Submit_1&quot;, {&quot;async&quot;:true,&quot;json&quot;:false,&quot;url&quot;:&quot;/inspireapp/Blacklist,searchComponent.searchForm.sdirect?updateParts=searchComponent&quot;});
                
                
            
            
        };
        tapestry.connect(&quot;Submit_1&quot;, &quot;onclick&quot;, &quot;event1707223370&quot;);
tapestry.cleanConnect(&quot;Submit_2&quot;, &quot;onclick&quot;, &quot;event1707223464&quot;);
        tapestry.event1707223464=function(e){
            tapestry.event.stopEvent(e);
            
                
                    tapestry.form.submit(&quot;searchForm&quot;, &quot;Submit_2&quot;, {&quot;async&quot;:true,&quot;json&quot;:false,&quot;url&quot;:&quot;/inspireapp/Blacklist,searchComponent.searchForm.sdirect?updateParts=searchComponent&quot;});
                
                
            
            
        };
        tapestry.connect(&quot;Submit_2&quot;, &quot;onclick&quot;, &quot;event1707223464&quot;);

tapestry.cleanConnect(&quot;sCriteria1&quot;, &quot;onchange&quot;, &quot;formEvent892686733&quot;);
                tapestry.formEvent892686733=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;Blacklist/searchComponent.sCriteria1&quot;, bcomponentid:&quot;sCriteria1&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;sCriteria1&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;sCriteria1&quot;, &quot;onchange&quot;, &quot;formEvent892686733&quot;);
tapestry.cleanConnect(&quot;comparator1&quot;, &quot;onchange&quot;, &quot;formEvent55030807&quot;);
                tapestry.formEvent55030807=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;Blacklist/searchComponent.comparator1&quot;, bcomponentid:&quot;comparator1&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;comparator1&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;comparator1&quot;, &quot;onchange&quot;, &quot;formEvent55030807&quot;);
tapestry.cleanConnect(&quot;sCriteria2&quot;, &quot;onchange&quot;, &quot;formEvent1403150071&quot;);
                tapestry.formEvent1403150071=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;Blacklist/searchComponent.sCriteria2&quot;, bcomponentid:&quot;sCriteria2&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;sCriteria2&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;sCriteria2&quot;, &quot;onchange&quot;, &quot;formEvent1403150071&quot;);
tapestry.cleanConnect(&quot;comparator2&quot;, &quot;onchange&quot;, &quot;formEvent255400393&quot;);
                tapestry.formEvent255400393=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;Blacklist/searchComponent.comparator2&quot;, bcomponentid:&quot;comparator2&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;comparator2&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;comparator2&quot;, &quot;onchange&quot;, &quot;formEvent255400393&quot;);
tapestry.cleanConnect(&quot;browseCriteria&quot;, &quot;onchange&quot;, &quot;formEvent151438188&quot;);
                tapestry.formEvent151438188=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;Blacklist/searchComponent.browseCriteria&quot;, bcomponentid:&quot;browseCriteria&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;browseCriteria&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;browseCriteria&quot;, &quot;onchange&quot;, &quot;formEvent151438188&quot;);
closeDialogComponent(&quot; , &quot;'&quot; , &quot;TinreadDialog&quot; , &quot;'&quot; , &quot;);

try {
  attachFocus(&quot; , &quot;'&quot; , &quot;field1&quot; , &quot;'&quot; , &quot;);
 }
 catch(e) {}
closeDialogComponent(&quot; , &quot;'&quot; , &quot;AssignedReports&quot; , &quot;'&quot; , &quot;);
closeDialogComponent(&quot; , &quot;'&quot; , &quot;ReportStatus&quot; , &quot;'&quot; , &quot;);
if(tapestry.fx) {
			var isResponse;
			tapestry.fx.attachAjaxStatus(function statusListener(status){
            	var fullDiv = document.getElementById(&quot; , &quot;'&quot; , &quot;ajaxLoaderDiv&quot; , &quot;'&quot; , &quot;);
            	var processDiv = document.getElementById(&quot; , &quot;'&quot; , &quot;pressingDiv&quot; , &quot;'&quot; , &quot;);
            	isResponse = status;
            	if(isResponse == false){
            		fullDiv.style.display=&quot;none&quot;;
            		processDiv.style.display=&quot;none&quot;;
            	}else{
            		fullDiv.style.display=&quot;&quot;;
            		setTimeout(
						function showProcessDiv(){
							if(isResponse==true){
								var processDiv = document.getElementById(&quot; , &quot;'&quot; , &quot;pressingDiv&quot; , &quot;'&quot; , &quot;);
								processDiv.style.display=&quot;&quot;;
							}
						}, 
						2000
					);
            	}
            }); 
		}
closeDialogComponent(&quot; , &quot;'&quot; , &quot;TinreadErrorDialog&quot; , &quot;'&quot; , &quot;);
closeDialogComponent(&quot; , &quot;'&quot; , &quot;TinreadMessageDialog&quot; , &quot;'&quot; , &quot;);});
// -->


id(&quot;orderby&quot;)&quot;) or . = concat(&quot;

dojo.registerModulePath(&quot;tapestry&quot;, &quot;/inspireapp/assets/static/tapestry-4.1.6&quot;);



dojo.require(&quot;tapestry.namespace&quot;);
tapestry.requestEncoding=&quot; , &quot;'&quot; , &quot;UTF-8&quot; , &quot;'&quot; , &quot;;















































&lt;!--
// directory of where all the images are
var cmThemePanelBase = &quot; , &quot;'&quot; , &quot;/inspireapp/ThemePanel/&quot; , &quot;'&quot; , &quot;;

var cmThemePanel =
{

  	// main menu display attributes
  	//
  	// Note.  When the menu bar is horizontal,
  	// mainFolderLeft and mainFolderRight are
  	// put in &lt;span>&lt;/span>.  When the menu
  	// bar is vertical, they would be put in
  	// a separate TD cell.

  	// HTML code to the left of the folder item
  	mainFolderLeft: &quot; , &quot;'&quot; , &quot;&lt;img alt=&quot;&quot; src=&quot;&quot; , &quot;'&quot; , &quot; + cmThemePanelBase + &quot; , &quot;'&quot; , &quot;blank.gif&quot;/>&quot; , &quot;'&quot; , &quot;,
  	// HTML code to the right of the folder item
  	mainFolderRight: &quot; , &quot;'&quot; , &quot;&lt;img alt=&quot;&quot; src=&quot;&quot; , &quot;'&quot; , &quot; + cmThemePanelBase + &quot; , &quot;'&quot; , &quot;arrowdown.gif&quot;/>&quot; , &quot;'&quot; , &quot;,
	// HTML code to the left of the regular item
	mainItemLeft: &quot; , &quot;'&quot; , &quot;&lt;img alt=&quot;&quot; src=&quot;&quot; , &quot;'&quot; , &quot; + cmThemePanelBase + &quot; , &quot;'&quot; , &quot;blank.gif&quot;/>&quot; , &quot;'&quot; , &quot;,
	// HTML code to the right of the regular item
	mainItemRight: &quot; , &quot;'&quot; , &quot;&lt;img alt=&quot;&quot; src=&quot;&quot; , &quot;'&quot; , &quot; + cmThemePanelBase + &quot; , &quot;'&quot; , &quot;blank.gif&quot;/>&quot; , &quot;'&quot; , &quot;,

	// sub menu display attributes

	// HTML code to the left of the folder item
	folderLeft: &quot; , &quot;'&quot; , &quot;&lt;img alt=&quot;&quot; src=&quot;&quot; , &quot;'&quot; , &quot; + cmThemePanelBase + &quot; , &quot;'&quot; , &quot;blank.gif&quot;/>&quot; , &quot;'&quot; , &quot;,
	// HTML code to the right of the folder item
	folderRight: &quot; , &quot;'&quot; , &quot;&lt;img alt=&quot;&quot; src=&quot;&quot; , &quot;'&quot; , &quot; + cmThemePanelBase + &quot; , &quot;'&quot; , &quot;arrow.gif&quot;/>&quot; , &quot;'&quot; , &quot;,
	// HTML code to the left of the regular item
	itemLeft: &quot; , &quot;'&quot; , &quot;&lt;img alt=&quot;&quot; src=&quot;&quot; , &quot;'&quot; , &quot; + cmThemePanelBase + &quot; , &quot;'&quot; , &quot;blank.gif&quot;/>&quot; , &quot;'&quot; , &quot;,
	// HTML code to the right of the regular item
	itemRight: &quot; , &quot;'&quot; , &quot;&lt;img alt=&quot;&quot; src=&quot;&quot; , &quot;'&quot; , &quot; + cmThemePanelBase + &quot; , &quot;'&quot; , &quot;blank.gif&quot;/>&quot; , &quot;'&quot; , &quot;,
	// cell spacing for main menu
	mainSpacing: 0,
	// cell spacing for sub menus
	subSpacing: 0,
	// auto dispear time for submenus in milli-seconds
	delay: 500
};

// for sub menu horizontal split
var cmThemePanelHSplit = [_cmNoClick, &quot; , &quot;'&quot; , &quot;&lt;td colspan=&quot;3&quot; style=&quot;height: 5px; overflow: hidden&quot;>&lt;div class=&quot;ThemePanelMenuSplit&quot;>&lt;/div>&lt;/td>&quot; , &quot;'&quot; , &quot;];
// for vertical main menu horizontal split
var cmThemePanelMainHSplit = [_cmNoClick, &quot; , &quot;'&quot; , &quot;&lt;td colspan=&quot;3&quot; style=&quot;height: 5px; overflow: hidden&quot;>&lt;div class=&quot;ThemePanelMenuSplit&quot;>&lt;/div>&lt;/td>&quot; , &quot;'&quot; , &quot;];
// for horizontal main menu vertical split
var cmThemePanelMainVSplit = [_cmNoClick, &quot; , &quot;'&quot; , &quot;|&quot; , &quot;'&quot; , &quot;];






function restore(idx) {
		var element = document.getElementById(&quot;restore&quot;);
		var href = element.href+&quot;&amp;sp=l&quot;+idx;
		tapestry.linkOnClick(href,&quot; , &quot;'&quot; , &quot;restore&quot; , &quot;'&quot; , &quot;, false);
    }  
	var clearUrl=null;
    function deleteBibliographic(idx, warning) {
		var element = document.getElementById(&quot;deleteBibliographic&quot;);
		if (clearUrl == null) {
			clearUrl= element.href;
		}
		element.href = clearUrl+&quot;&amp;sp=l&quot;+idx+&quot;&amp;sp=&quot;+warning;
		clickDirectLink(&quot;deleteBibliographic&quot;);
    }
dojo.require(&quot;tapestry.event&quot;);




function openDialogComponent(compId, hideCloseIcon, hideMaximizeIcon, closeHiddenForm) {
  	if(closeHiddenForm!=null &amp;&amp; !String(closeHiddenForm)==&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;){
	    if(compId) {
	      var d = new MyAppAlert(compId, hideCloseIcon, hideMaximizeIcon, &quot; , &quot;'&quot; , &quot;/inspireapp/images/&quot; , &quot;'&quot; , &quot;, closeHiddenForm);
	    }
    }else{
    	if(compId) {
	      var d = new MyAppAlert(compId, hideCloseIcon, hideMaximizeIcon, &quot; , &quot;'&quot; , &quot;/inspireapp/images/&quot; , &quot;'&quot; , &quot;);
	    }
    }
  }
  function closeDialogComponent(compId) {
    if(compId) {
        MyAppAlert.closeDialog(compId);
    }
  }
function printItems(href) {
	hrefparameters = addCheck();
	if (hrefparameters!=null){

		popupwindow = window.open(&quot;&quot; ,&quot;printRecord&quot;, &quot;status=no,toolbar=0,scrollbars=yes,menubar=0,titlebar=0,resizable=0,width=1500,height=1500&quot;);
		popupwindow.moveTo(screen.width/2-325, screen.height/2-235);
		popupwindow.focus();
		popupwindow.location = href+hrefparameters;
	}
	else{
		var buttonObject = window.parent.document.getElementById(&quot;InfoMsg&quot;);
	  	buttonObject.onclick();
	}
	return false;
}

function addCheck(){
	var formObject = document.getElementById(&quot;searchForm&quot;);
	total=0;
	href=&quot;&quot;;
	for(var i=0;i &lt; formObject.length;i++) {
		var item = formObject.elements[i];
		if (item.name.indexOf(&quot;selectat&quot;) == 0) {
			if (item.checked == true) {
					id = formObject.elements[i-1];
					if (total == 0) delimitator = &quot;?&quot;;
					else delimitator = &quot;&amp;&quot;;
					href = href+delimitator+&quot;sp=&quot;+id.value;
					total++;					
			}
		}
	}
	if (total>0) return href; 
	else return null;
}

 
 
 function attachFocus(fieldName) {
    var field = document.getElementById(fieldName);
    field.focus();
    VirtualKeyboard.attachInput(field);
}
  
 
  imagesPath = &quot; , &quot;'&quot; , &quot;/inspireapp/images/&quot; , &quot;'&quot; , &quot;;
  
   function clickSearchButton() {
    var searchDiv = document.getElementById(&quot;HiddenSearch&quot;);
    var browseDiv = document.getElementById(&quot;HiddenBrowse&quot;);
    if (searchDiv) {
     if (browseDiv) {
	    if (searchDiv.style.display == &quot;none&quot;) {
	      var browseButton = document.getElementById(&quot;browse&quot;);
	      browseButton.click();
	    }
	    else {
		    var searchButton = document.getElementById(&quot;formSubmitSearch&quot;);
		    searchButton.click();
	    }
     }
    }
    else {
     if (browseDiv) {
          var browseButton = document.getElementById(&quot;browse&quot;);
	      browseButton.click();
     }
     else {
		    var searchButton = document.getElementById(&quot;formSubmitSearch&quot;);
		    searchButton.click();
     } 
    }
  }


function changeScheduledStatusReport(){
	var checkBox = document.getElementById(&quot;scheduledReport&quot;);
	var scheduledData = document.getElementById(&quot;ScheduledData&quot;);
	var sheduleImg = document.getElementById(&quot;sheduleImg&quot;);
	
	if (checkBox.checked==false) {
	    checkBox.checked = true;
	    scheduledData.style.display =&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	    sheduleImg.src =  &quot;images/arrow_checked.gif&quot;;
	  }
	else {
	 checkBox.checked = false;
	 scheduledData.style.display =&quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;
	 sheduleImg.src =  &quot;images/arrow_unchecked.gif&quot;;
   }
}
dojo.require(&quot;tapestry.fx&quot;);
// -->


	
	
	







		
	
	
		
			
				
				Hi, catc (CU)
				
				最近登入:2024-03-22 15:26:59 ,成功
				
				
					
					
                       0 筆新訊息
                    
                
				
				繁體中文English繁體中文
				
English
繁體中文
						
						
				
				
					
					登 出
				
			
		
		
		
			
			
				神資圖書館(15trunk機)
			
			
		
	
	
	
	
	
		
		
		
			
			
			 查詢  編目  流通  採購   期刊控制   場地設備  說明  我的帳戶  學科服務  管理  統計  清單  Portal  臉辨報表  ERM 
			簡單查詢進階查詢FRBR查詢最近的查詢已儲存的記錄FRBR作品書目書目記錄維護註記刪除之書目記錄書目記錄全域修改權威記錄權威記錄維護註記刪除之權威記錄權威記錄全域修改條碼號館藏資料維護註記刪除之館藏資料維護出版者出版者資料維護內部移送作業書單維護FRBR作品辦證讀者記錄維護檢查讀者註記刪除之讀者記錄讀者證卡記錄維護離職離校借閱檯讀者流通檯還書箱讀者查詢交易紀錄違規讀者查詢讀者個人違規記錄指定參考書交易歷史查詢一般搜尋交易館藏預約順位調整題名預約順位調整批次修改到期日內部移送作業盤點盤點維護跨館預約/調閱跨館預約/調閱需求移轉-寄送項目移轉-收到項目盤點比對批次報廢選擇報廢執行報廢已報廢歷史查詢書箱作業條碼號空號清單(條碼機印製)訂購檢查薦購請購訂購明細訂單登收用卷期登收程序發票款項維護預算經費供應商標案內部移送作業催缺候選催缺項目催缺記錄期刊主檔訂購檢查薦購請購核可訂購明細訂單登收用卷期登收程序發票款項維護預算經費供應商裝訂裝訂主檔維護擬裝設定送裝確認裝訂登收標案內部移送作業催缺候選催缺項目催缺記錄流通記錄預約記錄借用記錄維護分類維護單項維護參數設定流通政策維護預約審核場地設備黑名單書籤訊息排程工作日誌學科館藏館藏資料類型設定學科館藏維護學科預約教師資料維護讀者評價選單學科預約維護學科預約查詢教師評語學科報表學科館藏冊數統計教師學科服務時數與使用人次統計學科服務讀者清單學科報表學科館藏冊數統計教師學科服務時數與使用人次統計學科服務讀者清單一般設定類型/使用者(館員)片語設定系統設定貨幣匯率館藏地範本設定--依館員帳號翻譯轉換 選取清單格式屬性條碼號規範顯示設定圖書館系統屬性推薦範本設定館員新增申請QR Code 設定編目書目類型設定MARC規範格式設定顯示設定停用字SDI維護同義字條碼號原則對應流通流通通知單範本流通通知單參數設定流通通知單發送紀錄日常作業排程一般每周開閉館設定閉館日期維護交易紀錄類型交易方式可適用讀者身份交易方式可適用館藏流通類別交易方式可適用使用類別交易紀錄-館藏狀態流通政策讀者證卡原則單位所系夾書單欄位設定讀者閱讀興趣發送紀錄期刊週期傳閱清單維護 轉入Z39屬性設定更新書封上傳書封索引編修索引重建功能維護公佈欄維護意見回覆維護主題維護特別書籍維護心得分享東區互惠審核東區互惠匯出系統代碼維護貨幣代碼作者號維護科特號維護OPACSDI維護讀者評論管理Help維護熱門關鍵字過濾館員Log記錄館員登入/出記錄館員資料變更記錄讀者資料變更記錄權限設定MARC權限設定功能權限設定角色設定系統狀態排程管理排程監控者排程日誌查詢統計檢索點/檢索語統計報表不同時段使用次數統計書目下載次數統計電子資源統計電子資源-館藏點閱次數統計電子資源-讀者點閱次數統計電子資源-單位系所點閱次數統計編目統計館藏量統計報表類號區間統計報表圖書增減統計報表編目/館別/大類交叉分析報表大類/冊數統計表大類/特藏交叉統計表特藏/冊數統計表新書特藏大類交叉統計圖書增減表／月份／大類大類語言種數冊數統計大類/條碼號類別冊數統計圖書冊數分類統計單圖書冊數金額分類統計非書資料數量金額統計資料類型範圍館藏統計表編目工作量統計(個人)編目工作量統計(各館)館藏狀態統計報表流通統計讀者借閱排行榜書展借閱清單流通人數統計(學制分頁 符合科系)讀者類型借閱暨歸還流通量統計報表流通人次冊數統計  利用概況統計(日報表)讀者借閱交叉分析各館特藏預約統計報表館內使用統計讀者身份借閱統計讀者個人及單位所系借閱排行榜館藏資料借閱統計單位系所借閱排行榜單位系所借閱統計熱門借閱排行榜熱門預約排行榜讀者辦証暨補發人次統計表借閱交叉分析統計資料類型流通統計館員流通量統計單位(個人)保管圖書(非書)數量明細館際互借代還統計-本館館藏館際互借代還統計-他館館藏指定參考書借閱量統計表各大類熱門借閱排行榜大專院校圖書館讀者身份借閱清單及統計報表館藏流通量統計跨館預約/調閱館藏借閱排行榜罰款收款人數/次數統計跨館預約/調閱使用量統計罰款收款統計特定館藏借閱統計採購統計書商採購統計年度經費採購統計特藏訂購方式金額統計書商到書率書商績效統計期刊統計期刊到刊率統計報表平均收刊日統計報表歷年期刊借閱統計書商到書率書商催缺次數書商累計訂購次數期刊訂費漲幅統計逾期違約金統計期刊經費訂購統計報表場地設備場地設備使用統計表報表模組範本維護編目清單書目標籤報表Tag856檢查報表新版書標維護次分類交叉分析登錄簿條碼產生清單登錄簿清單條碼號空號清單單位科系館藏設定清單館藏狀態預約清單特殊館藏清單館藏狀態清單報表流通清單罰款未收款報表預約待取清單借閱(逾期)清單還書清單新書清單讀者欠款清單借閱逾期清單罰款收款清單預約撤架清單長期借閱圖書清單離線流通清單預約額滿資料清單單位系所借閱逾期清單跨館還書清單離職離校清單跨館移送清單預約保留結束清單指定參考書借閱量明細表未外借館藏清單重複讀者比對報表借閱逾期通知單取書櫃上架清單預約圖書清單採購清單年度經費採買書籍運用率訂購資料明細表擬購資料明細表請購單詢價單訂購逾期清單謝函書的催缺報表薦購書刊複本比對報表採購參考清單財產增加清冊財產減少清冊已付款未到清單訂購明細期刊清單期刊合訂本登錄簿交贈期刊清單期刊架位清單裝訂歷史清單薦購書刊複本比對報表期刊登錄簿索贈清單期刊查驗清單已付款未到清單場地設備預約取消清單臉辨用戶使用分析表臉辨用戶與圖書分類關係表期間辦證統計_年報期間辦證統計_月報期間辦證明細期間辦證異動統計_年報期間辦證異動統計_月報辦證異動明細臉辨使用紀錄統計_年報臉辨使用紀錄統計_月報臉辨使用紀錄明細人數統計表人數統計明細表人次統計表人次統計明細表熱門時段統計表讀者進出排行榜期間辦證刪除明細系統參數設定IP區間管理延伸查詢設定系統設定檔資源瀏覽/後分類設定資源到期通知設定可使用身分類別設定 代碼管理所屬資料庫清單共用代碼檔共用代碼類別電子資源管理資源前端顯示設定Ezproxy設定檔維護電子資料庫/網路資源電子期刊/電子書電子資源批次匯入電子資源副檔匯入讀者回報資源連線異常 Meniuri.erm.9000報表點閱資源排行榜依學院別使用統計依單位別使用統計依月分資源使用統計依學院登入統計表依系所登入統計表依身分登入統計表登入清單依身分別使用統計表使用者使用資源清單 收藏資源排行榜偵測電子資源狀態設定偵測資源時間設定偵測電子資源查詢電子資源狀態電子資源狀況報表
				
	


		
			
				場地設備 > 場地設備黑名單		
			
	


							
	jQuery(function () {
		jQuery(&quot;div[id=&quot; , &quot;'&quot; , &quot;hiddenDiacritics&quot; , &quot;'&quot; , &quot;]&quot;).draggable({
	        containment: &quot;#box&quot;,
	        scroll: false
	});
	});
	jQuery(function () {
		jQuery(&quot;div[id=&quot; , &quot;'&quot; , &quot;symb_div&quot; , &quot;'&quot; , &quot;]&quot;).draggable({
			containment: &quot;#box&quot;,
			scroll: false
	});
	});


function checkElement(elementId, checkedElem){
	href = document.getElementById(&quot;checkInform&quot;).href+&quot; , &quot;'&quot; , &quot;&amp;sp=l&quot; , &quot;'&quot; , &quot;+elementId+&quot; , &quot;'&quot; , &quot;&amp;sp=&quot; , &quot;'&quot; , &quot;+(checkedElem==true?&quot; , &quot;'&quot; , &quot;T&quot; , &quot;'&quot; , &quot;:&quot; , &quot;'&quot; , &quot;F&quot; , &quot;'&quot; , &quot;);
	return tapestry.linkOnClick(href,&quot; , &quot;'&quot; , &quot;checkInform&quot; , &quot;'&quot; , &quot;, false);
}

function syncWithOrder(){
document.getElementById(&quot;orderCriteria&quot;).value = document.getElementById(&quot;browseCriteria&quot;).value;
}
function syncWithBrowse(){
document.getElementById(&quot;browseCriteria&quot;).value = document.getElementById(&quot;orderCriteria&quot;).value;
}

var refreshTime = 0;
function apasa() {
  refreshTime = 2500;

  clickLinkSubmit(&quot;searchForm&quot;, &quot;scriptSubmit&quot;);
}

function showingOrderBy(status){
	document.getElementById(&quot;showOrderBy&quot;).style.display = status;
}
function remakeSearch(){
	if(typeof(tapestry.form)!=&quot;undefined&quot;) clickLinkSubmit(&quot; , &quot;'&quot; , &quot;searchForm&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;remakeSearch&quot; , &quot;'&quot; , &quot;);
	else window.setTimeout(&quot;remakeSearch()&quot;,500);
}

function remakePaginatedSearch(){
	clickLinkSubmit(&quot; , &quot;'&quot; , &quot;searchForm&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;remakePaginatedSearch&quot; , &quot;'&quot; , &quot;);
}
j(document).ready(function(){
	j(&quot;#resetbutton&quot;).click(function(){	
		//j(&quot;#reseter&quot;).click();
		var h=j(&quot;#reseter&quot;).attr(&quot;href&quot;);
		window.location=h;
	});
	
    // JSON 資料結構模擬  start
    try{
    var treeJsonData = JSON.parse(j(&quot; , &quot;'&quot; , &quot;#locData&quot; , &quot;'&quot; , &quot;).text());
    
    j(&quot; , &quot;'&quot; , &quot;.place&quot; , &quot;'&quot; , &quot;).treeoptions({
        data: treeJsonData,
        openImg: &quot; , &quot;'&quot; , &quot;/inspireapp/images/ico_add.gif&quot; , &quot;'&quot; , &quot;, // img 路徑
        cleanImg:&quot; , &quot;'&quot; , &quot;/inspireapp/images/clear.gif&quot; , &quot;'&quot; , &quot; // img 路徑
    });
    }catch(e){
    }
    try{
    var treeJsonData1 = JSON.parse(j(&quot; , &quot;'&quot; , &quot;#itemClassData&quot; , &quot;'&quot; , &quot;).text());
    
    j(&quot; , &quot;'&quot; , &quot;.place0&quot; , &quot;'&quot; , &quot;).treeoptions({
        data: treeJsonData1,
        openImg: &quot; , &quot;'&quot; , &quot;/inspireapp/images/ico_add.gif&quot; , &quot;'&quot; , &quot;, // img 路徑
        cleanImg:&quot; , &quot;'&quot; , &quot;/inspireapp/images/clear.gif&quot; , &quot;'&quot; , &quot; // img 路徑
    });
    }catch(e){
    }
    try{
    var treeJsonData2 = JSON.parse(j(&quot; , &quot;'&quot; , &quot;#materialTypeData&quot; , &quot;'&quot; , &quot;).text());
    
    j(&quot; , &quot;'&quot; , &quot;.place1&quot; , &quot;'&quot; , &quot;).treeoptions({
        data: treeJsonData2,
        openImg: &quot; , &quot;'&quot; , &quot;/inspireapp/images/ico_add.gif&quot; , &quot;'&quot; , &quot;, // img 路徑
        cleanImg:&quot; , &quot;'&quot; , &quot;/inspireapp/images/clear.gif&quot; , &quot;'&quot; , &quot; // img 路徑
    });
    }catch(e){
    }
    try{
        var treeJsonData3 = JSON.parse(j(&quot; , &quot;'&quot; , &quot;#languageData&quot; , &quot;'&quot; , &quot;).text());
        
        j(&quot; , &quot;'&quot; , &quot;.place2&quot; , &quot;'&quot; , &quot;).treeoptions({
            data: treeJsonData3,
            openImg: &quot; , &quot;'&quot; , &quot;/inspireapp/images/ico_add.gif&quot; , &quot;'&quot; , &quot;, // img 路徑
            cleanImg:&quot; , &quot;'&quot; , &quot;/inspireapp/images/clear.gif&quot; , &quot;'&quot; , &quot; // img 路徑
        });
    }catch(e){
    }
    try{
        var treeJsonData4 = JSON.parse(j(&quot; , &quot;'&quot; , &quot;#langCataData&quot; , &quot;'&quot; , &quot;).text());
        
        j(&quot; , &quot;'&quot; , &quot;.place3&quot; , &quot;'&quot; , &quot;).treeoptions({
            data: treeJsonData4,
            openImg: &quot; , &quot;'&quot; , &quot;/inspireapp/images/ico_add.gif&quot; , &quot;'&quot; , &quot;, // img 路徑
            cleanImg:&quot; , &quot;'&quot; , &quot;/inspireapp/images/clear.gif&quot; , &quot;'&quot; , &quot; // img 路徑
        });
    }catch(e){
    }
    
    try{
        var departmentJson = JSON.parse(j(&quot; , &quot;'&quot; , &quot;#departmentData&quot; , &quot;'&quot; , &quot;).text());
        
        j(&quot; , &quot;'&quot; , &quot;.multipleDepartmentClass&quot; , &quot;'&quot; , &quot;).treeoptions({
            data: departmentJson,
            openImg: &quot; , &quot;'&quot; , &quot;/inspireapp/images/ico_add.gif&quot; , &quot;'&quot; , &quot;,
            cleanImg:&quot; , &quot;'&quot; , &quot;/inspireapp/images/clear.gif&quot; , &quot;'&quot; , &quot;
        });
    }catch(e){}
});

function runScript(e) {
    if (e.keyCode == 13) {
        document.getElementById(&quot;browse&quot;).click();
        return false;
    }
}

function createPopEdit(href) {

	popupwindow = window.open(&quot;&quot; ,&quot;MeniuCatalogare&quot;, &quot;status=no,toolbar=0,scrollbars=yes,menubar=0,titlebar=0,resizable=1,width=1500,height=1500&quot;);
	popupwindow.moveTo(screen.width/2-435, screen.height/2-300);
	popupwindow.focus();

	popupwindow.location = href;

	if (popupwindow == null) popupwindow.opener = self;
	return false;

}

function createUploadPopEdit(href) {
	popupwindow = window.open(&quot;&quot; ,&quot;Upload&quot;, &quot;status=no,toolbar=0,scrollbars=yes,menubar=0,titlebar=0,resizable=1,top=&quot; + (screen.height-150)/2 + &quot;,left=&quot; + (screen.width-600)/2 + &quot;,width=600,height=250&quot;);
	popupwindow.focus();

	popupwindow.location = href;

	if (popupwindow == null) popupwindow.opener = self;
	return false;
};



	
	
		
			refreshTime = 0;
		
	



			 
				
						
						查詢
							
						
						
				   
				
						
							瀏覽
								
							
						
				
				
	  		
					
						
						 
							新增 
		
				
				
				
				
				
				
				
				
				
				















































	

	
	
	
	
		
			
			查詢條件
				
					
日期
讀者姓名
讀者證號

				
					
起始以
包含
等於(=)
大於(>=)
小於(&lt;=)
不等於(≠)

				
				
				
					
					
						
						
							
						
					
				
				
				
				
					
and
or
not

					
日期
讀者姓名
讀者證號

				
					
起始以
包含
等於(=)
大於(>=)
小於(&lt;=)
不等於(≠)

				
				
				
					
					
						
						
							
						
					
				
				
				
				
				
				
					
								
		                    
		                    	
								
				
			
			
				
				
				  

					
				

             
			
			
			
				
		
	

	
	
	
		瀏覽條件:
		
			
日期
讀者姓名
讀者證號

		
		
			
			
			
			起始以
			
			:
		
		
			
			
				 
				
				

//&lt;![CDATA[

					document.getElementById(&quot;listField&quot;).focus();
				
//]]&gt;


			
		
	    
		 
			
      
	
	 
	
	
    
		            
		        
		              

		          
			 

			
			
  					
					 
					     
					      
					       
					       
					       
					 		
						                  排序條件 :  
							  
							  
日期
讀者姓名
讀者證號

						       
						       
						      
						      
						   
							
								
升冪
降冪

							 
							
								
								
								



                            	
                              		
                            	
                            	
							
						
						

					 
					 
					   
         
			 
			 
			 
             
                
                    
                      
                        符號表:
                        
                      
                      
                        
貨幣：
￥   
￡   
₤
₣
₢      
₡
₠
₥
₦
₧ 
₨
₩
₪
₫
€

數學符號：
±
Ω
λ
β
α
θ
π
μ
≠
≤
≥
∑

日文：
あ
い
う
え
お
か
き
く
け
こ
さ
し

す
せ
そ
た
ち
つ
て
と
な
に
ぬ
ね

の
は
ひ
ふ
へ
ほ
ま
み
む
め
も
や

ゆ
よ
ら
り
る
れ
ろ
わ
を
ん

が
ぎ

ぐ
げ
ご
ざ
じ
ず
ぜ
ぞ
だ
ぢ
づ
で

ど
ば
び
ぶ
べ
ぼ

ぱ
ぴ
ぷ
ぺ
ぽ


ア
イ
ウ
エ
オ
カ
キ
ク
ケ
コ
サ
シ

ス
セ
ソ
タ
チ
ツ
テ
ト
ナ
ニ
ヌ
ネ

ノ
ハ
ヒ
フ
ヘ
ホ
マ
ミ
ム
メ
モ
ヤ

ユ
ヨ
ラ
リ
ル
レ
ロ
ワ
ヲ
ン
ガ
ギ

グ
ゲ
ゴ
ザ
ジ
ズ
ゼ
ゾ
ダ
ヂ
ヅ
デ

ド
バ
ビ
ブ
ベ
ボ
パ
ピ
プ
ペ
ポ

ぃ
ぅ
ぇ
ぉ
っ
ゃ
ゅ
ょ
ゎ
ァ
ィ
ゥ

ェ
ォ
ッ
ャ
ュ
ョ
ヮ
々

羅馬數字：
Ⅰ
Ⅱ
Ⅲ
Ⅳ
Ⅴ
Ⅵ
Ⅶ
Ⅷ
Ⅸ
Ⅹ


                      
                    
                
                
             
			 
			 
  

  
			
		
		
		 
10
20
50
100

      	 
      	 
		 
		
	
  
  	

	
     	 
		



	

	

		
	
		 


 
 
 
 



 
  
    Go To Page
  
  
 
 
  
 




		  
		    
		  
		  
		  



		  {
  &quot;data&quot; : [  
{
  &quot;name&quot; : &quot;CMUL - 神資圖書館&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;2 - 2&quot;},
{ &quot;name&quot; : &quot;123 - 123&quot;},
{ &quot;name&quot; : &quot;20230417 - 20230417&quot;},
{ &quot;name&quot; : &quot;20230418 - 20230418&quot;},
{
  &quot;name&quot; : &quot;AH - 安南醫院&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;AHGL - 安南圖書區&quot;}]
},
{ &quot;name&quot; : &quot;av - av&quot;},
{ &quot;name&quot; : &quot;B007 - B007&quot;},
{ &quot;name&quot; : &quot;BCSB4 - BCSB4&quot;},
{ &quot;name&quot; : &quot;BX - 取書櫃1&quot;},
{ &quot;name&quot; : &quot;BY - 取書櫃2&quot;},
{
  &quot;name&quot; : &quot;CB - 北港分館&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;BAVN - 北港分館視聽區(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;BCIR - 北港分館流通櫃檯&quot;},
{ &quot;name&quot; : &quot;BCRA - 北港分館指參(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;BPAV - 北港分館視聽區&quot;},
{ &quot;name&quot; : &quot;BPCL - 北港分館書庫&quot;}]
},
{ &quot;name&quot; : &quot;cbook - cbook&quot;},
{ &quot;name&quot; : &quot;circd - circd&quot;},
{ &quot;name&quot; : &quot;clp - clp&quot;},
{
  &quot;name&quot; : &quot;CM - 北港附設醫院&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;BMHL - 北港附設醫院圖書室&quot;}]
},
{ &quot;name&quot; : &quot;CMUL - 中國醫藥大學圖書館 - CMUL - 中國醫藥大學圖書館&quot;},
{
  &quot;name&quot; : &quot;CU - 台中總館&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;CUAV - 台中總館視聽區(獨立專區)&quot;},
{ &quot;name&quot; : &quot;MAVN - 台中總館視聽區(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MAVR - 台中總館視聽區&quot;},
{ &quot;name&quot; : &quot;MCAT - 台中總館技服組&quot;},
{ &quot;name&quot; : &quot;MCBS - 台中總館密閉書庫&quot;},
{ &quot;name&quot; : &quot;MCIR - 台中總館流通櫃檯&quot;},
{ &quot;name&quot; : &quot;MCRA - 台中總館教師指定參考書(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MCSS - 台中總館B3裝訂期刊區&quot;},
{ &quot;name&quot; : &quot;MDIA - 台中總館博碩士論文區&quot;},
{ &quot;name&quot; : &quot;MEAS - 台中總館探索史懷哲之路專書區&quot;},
{ &quot;name&quot; : &quot;MEXM - 台中總館國考書區(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MFGA - 台中總館本校教職優良教材區&quot;},
{ &quot;name&quot; : &quot;MFPA - 台中總館本校教師升等資料區&quot;},
{ &quot;name&quot; : &quot;MFSA - 台中總館本校教職論著&quot;},
{ &quot;name&quot; : &quot;MHME - 台中總館人文專書區-醫學倫理(限館內閱覽)台中總館人文專書區-醫學倫理(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MHMH - 台中總館人文專書區-醫療史(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MHML - 台中總館人文專書區-醫學法律(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MHQC - 台中總館人文專書區-醫療品質(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MNBR - 台中總館新書展示區&quot;},
{ &quot;name&quot; : &quot;MPAA - 台中總館績效暨獲獎區(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MPBL - 台中總館PBL專書區(限館內閱覽)&quot;},
{ &quot;name&quot; : &quot;MREF - 台中總館參考室&quot;},
{ &quot;name&quot; : &quot;MRSS - 台中總館閱覽組&quot;},
{ &quot;name&quot; : &quot;MS-C - 台中總館期刊複本櫃&quot;},
{ &quot;name&quot; : &quot;MSER - 台中總館期刊區&quot;},
{ &quot;name&quot; : &quot;MSPA - 台中總館研究計劃專書&quot;},
{ &quot;name&quot; : &quot;MSPB - 中醫醫史文獻室(限所內閱覽)&quot;},
{ &quot;name&quot; : &quot;MSPC - 台中總館特藏室&quot;},
{ &quot;name&quot; : &quot;MSTK - 台中總館書庫&quot;},
{ &quot;name&quot; : &quot;MYBK - 台中總館參考壁櫃&quot;},
{ &quot;name&quot; : &quot;new item 7 - new item 7&quot;},
{ &quot;name&quot; : &quot;ONLN - 台中總館線上資料&quot;}]
},
{ &quot;name&quot; : &quot;e-resources - 電子資源&quot;},
{ &quot;name&quot; : &quot;EB-P - EB-P&quot;},
{ &quot;name&quot; : &quot;elect - elect&quot;},
{ &quot;name&quot; : &quot;H-EQ - H-EQ&quot;},
{ &quot;name&quot; : &quot;H-MR - H-MR&quot;},
{ &quot;name&quot; : &quot;L - L&quot;},
{ &quot;name&quot; : &quot;L40 - L40&quot;},
{ &quot;name&quot; : &quot;LB 圖書總館 - LB 圖書總館&quot;},
{ &quot;name&quot; : &quot;LB-S - LB-S&quot;},
{
  &quot;name&quot; : &quot;LE - 語文教學中心&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;LEGL - 語文教學中心圖書室&quot;}]
},
{ &quot;name&quot; : &quot;LIB - LIB&quot;},
{
  &quot;name&quot; : &quot;new item 1 - new item 1&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;new item 3 - new item 3&quot;}]
},
{ &quot;name&quot; : &quot;new item 10 - new item 10&quot;},
{ &quot;name&quot; : &quot;new item 12 - new item 12&quot;},
{ &quot;name&quot; : &quot;new item 13 - new item 13&quot;},
{ &quot;name&quot; : &quot;new item 14 - new item 14&quot;},
{ &quot;name&quot; : &quot;new item 16 - new item 16&quot;},
{ &quot;name&quot; : &quot;new item 2 - new item 2&quot;},
{ &quot;name&quot; : &quot;new item 20 - new item 20&quot;},
{ &quot;name&quot; : &quot;new item 4 - new item 4&quot;},
{ &quot;name&quot; : &quot;new item 5 - new item 5&quot;},
{ &quot;name&quot; : &quot;new item 6 - 英才校區&quot;},
{ &quot;name&quot; : &quot;new item 8 - new item 8&quot;},
{ &quot;name&quot; : &quot;new item 9 - new item 9&quot;},
{ &quot;name&quot; : &quot;NPTU - NPTU&quot;},
{ &quot;name&quot; : &quot;OUK - OUK&quot;},
{
  &quot;name&quot; : &quot;PT - 培德醫院&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;new item 11 - new item 11&quot;},
{ &quot;name&quot; : &quot;new item 17 - new item 17&quot;},
{ &quot;name&quot; : &quot;PTGL - 培德醫院圖書區&quot;}]
},
{ &quot;name&quot; : &quot;ptext - ptext&quot;},
{ &quot;name&quot; : &quot;SB3 - SB3&quot;},
{ &quot;name&quot; : &quot;T-P - T-P&quot;},
{ &quot;name&quot; : &quot;TBBK - TBBK&quot;},
{
  &quot;name&quot; : &quot;TH - 台北分院&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;THGL - 台北分院圖書區&quot;},
{ &quot;name&quot; : &quot;THPA - 台北分院期刊區&quot;}]
},
{ &quot;name&quot; : &quot;W-P - W-P&quot;},
{
  &quot;name&quot; : &quot;YH - 豐原分院&quot;,
  &quot;data&quot;: [

{ &quot;name&quot; : &quot;new item 18 - new item 18&quot;},
{ &quot;name&quot; : &quot;new item 19 - new item 19&quot;},
{ &quot;name&quot; : &quot;YHGL - 豐原分院圖書區&quot;}]
},
{ &quot;name&quot; : &quot;z3llc - z3llc&quot;},
{ &quot;name&quot; : &quot;z6bkf - z6bkf&quot;},
{ &quot;name&quot; : &quot;zd1a2 - zd1a2&quot;},
{ &quot;name&quot; : &quot;zd1e - zd1e&quot;},
{ &quot;name&quot; : &quot;zdlf - zdlf&quot;},
{ &quot;name&quot; : &quot;五樓漫畫書專區 - 五樓漫畫書專區&quot;},
{ &quot;name&quot; : &quot;實體館藏 - 實體館藏&quot;},
{ &quot;name&quot; : &quot;綜合書庫 - 綜合書庫&quot;},
{ &quot;name&quot; : &quot;艾迪訊圖書館 - 艾迪訊圖書館&quot;},
{ &quot;name&quot; : &quot;附中出版物專區 - 附中出版物專區&quot;},
{ &quot;name&quot; : &quot;龍華科技大學圖書館 - 龍華科技大學圖書館&quot;}]
}]
}
		  {&quot;data&quot; :[{&quot;name&quot; : &quot;B可借圖書&quot;},{&quot;name&quot; : &quot;B電子資源&quot;},{&quot;name&quot; : &quot;eee&quot;},{&quot;name&quot; : &quot;M可借行動設備&quot;},{&quot;name&quot; : &quot;P可借期刊&quot;},{&quot;name&quot; : &quot;V可借視聽&quot;},{&quot;name&quot; : &quot;www&quot;},{&quot;name&quot; : &quot;不流通&quot;},{&quot;name&quot; : &quot;書箱借閱30天&quot;}]}


         

	
	
		

// 	dojo.event.topic.subscribe(&quot; , &quot;'&quot; , &quot;info&quot; , &quot;'&quot; , &quot;, closeDialogComponent(&quot; , &quot;'&quot; , &quot;AssignedReports&quot; , &quot;'&quot; , &quot;));
	dojo.event.topic.subscribe(&quot; , &quot;'&quot; , &quot;info&quot; , &quot;'&quot; , &quot;,function(msg){closeDialogComponent(&quot; , &quot;'&quot; , &quot;AssignedReports&quot; , &quot;'&quot; , &quot;);});


  
 
  
    報表
  
  
 
 
  
 

	
	


	var refreshTimeout = null;
	function refresh() {
		clickDirectLink(&quot; , &quot;'&quot; , &quot;refresher&quot; , &quot;'&quot; , &quot;);
		setstytle();
	};
	function setstytle() {
		var ReportStatus = document.getElementById(&quot;ReportStatus&quot;);
		ReportStatus.style.width = &quot;663px&quot;;
		ReportStatus.style.height = &quot;360px&quot;;
	};



 
  
    報表狀態
  
  
 
 
  
 


	

	

			
		
	




		
				//alert(myMenu);
				//alert(JSON.stringify(cmThemePanel));
			//	cmRDraw (&quot; , &quot;'&quot; , &quot;myMenuID&quot; , &quot;'&quot; , &quot;, myMenu, &quot; , &quot;'&quot; , &quot;hbr&quot; , &quot;'&quot; , &quot;, cmThemePanel, &quot; , &quot;'&quot; , &quot;ThemePanel&quot; , &quot;'&quot; , &quot;);
			 createMenuStr(myMenu);
		
		
			
	
		
        
          
    			
    				
    					
    						    
    						    
    						    	處理中...  
    						    
    												
    				
    			
    		
        
				



		
			
 
  
  
     
  
 
 
  
 


			   
 
  
  
     
  
 
 
  
 

   



			


		
		
        
        
        
		
				
			 Copyright© 2016 iNspire 4.4.0-SNAPSHOT by Claridy Solutions, Inc. All rights reserved.
		
		
		
        
        
		
		  
	function inputKeyCode(){ 
		keyCode=123; 
		keyEnable=true; 
	} 

		 		 

      

(function($){

	$(document).ready(function(){
		var curL = Date.parse(new Date()).valueOf();
		var activeL = Date.parse($(&quot; , &quot;'&quot; , &quot;#activeTime&quot; , &quot;'&quot; , &quot;).text()).valueOf();
		var inactiveL = activeL+300000;
		
		if( curL > activeL &amp;&amp; inactiveL > curL ) {
			$(&quot; , &quot;'&quot; , &quot;.marquee&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;block&quot; , &quot;'&quot; , &quot;);
		}else{
			$(&quot; , &quot;'&quot; , &quot;.marquee&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;);
		}
		
		$(&quot; , &quot;'&quot; , &quot;.marquee&quot; , &quot;'&quot; , &quot;).marquee({
			duration: 8000
		});
	})
	
	function marqueeInit(){
		console.log(&quot; , &quot;'&quot; , &quot;關閉跑馬燈 &amp; 重新檢查&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.marquee&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;);
	}
	
})(jQuery);
 

	var keyCode;
	var keyEnable;
	document.onkeydown=KeyDown;
	function KeyDown(e){
		inputKeyCode();
		if(e.type ==&quot;keydown&quot;){
			var keyNum=window.event ? e.keyCode :e.which; 
		}else{
			var keyNum = e;
		}
			
		if(keyCode!=null &amp;&amp; keyEnable){
			if(keyNum==keyCode){
				createPopEdit(&quot;/inspireapp/UtilizatorPhraseDetails,$PopupBorder.$DirectLink_2.sdirect?updateParts=CloseReminderDialog&quot;);
			}	
		}
	}

	&lt;!--
tapestry.addOnLoad(function(e) {
dojo.require(&quot;tapestry.form&quot;);tapestry.form.registerForm(&quot;LanguageForm&quot;);

tapestry.form.registerForm(&quot;searchForm&quot;);
var field1 = new Ajax.Autocompleter(&quot;field1&quot;, &quot;field1choices&quot;, &quot;/inspireapp/Blacklist,searchComponent.field1.sdirect?sp=Sfield1&amp;sp=Snavigare24.valoare_afisare&amp;sp=Sstarts+with&amp;sp=1&amp;updateParts=field1&quot;, {&quot;method&quot;:&quot;get&quot;,&quot;frequency&quot;:0.2,&quot;minChars&quot;:1,&quot;onFailure&quot;:tapestry.error,&quot;encoding&quot;:&quot;UTF-8&quot;});
var field2 = new Ajax.Autocompleter(&quot;field2&quot;, &quot;field2choices&quot;, &quot;/inspireapp/Blacklist,searchComponent.field2.sdirect?sp=Sfield2&amp;sp=Scards.card_number&amp;sp=Sstarts+with&amp;sp=2&amp;updateParts=field2&quot;, {&quot;method&quot;:&quot;get&quot;,&quot;frequency&quot;:0.2,&quot;minChars&quot;:1,&quot;onFailure&quot;:tapestry.error,&quot;encoding&quot;:&quot;UTF-8&quot;});
tapestry.cleanConnect(&quot;formSubmitSearch&quot;, &quot;onclick&quot;, &quot;event1167465096&quot;);
        tapestry.event1167465096=function(e){
            tapestry.event.stopEvent(e);
            
                
                    tapestry.form.submit(&quot;searchForm&quot;, &quot;formSubmitSearch&quot;, {&quot;async&quot;:true,&quot;json&quot;:false,&quot;url&quot;:&quot;/inspireapp/Blacklist,searchComponent.searchForm.sdirect?updateParts=results&amp;updateParts=nrResults&amp;updateParts=autoRefreshZone&amp;updateParts=zSources&amp;updateParts=localSources&amp;updateParts=showOrderBy&quot;});
                
                
            
            
        };
        tapestry.connect(&quot;formSubmitSearch&quot;, &quot;onclick&quot;, &quot;event1167465096&quot;);
tapestry.cleanConnect(&quot;browse&quot;, &quot;onclick&quot;, &quot;event1984482014&quot;);
        tapestry.event1984482014=function(e){
            tapestry.event.stopEvent(e);
            
                
                    tapestry.form.submit(&quot;searchForm&quot;, &quot;browse&quot;, {&quot;async&quot;:true,&quot;json&quot;:false,&quot;url&quot;:&quot;/inspireapp/Blacklist,searchComponent.searchForm.sdirect?updateParts=results&amp;updateParts=nrResults&amp;updateParts=autoRefreshZone&amp;updateParts=zSources&amp;updateParts=localSources&quot;});
                
                
            
            
        };
        tapestry.connect(&quot;browse&quot;, &quot;onclick&quot;, &quot;event1984482014&quot;);
tapestry.cleanConnect(&quot;Submit&quot;, &quot;onclick&quot;, &quot;event178957379&quot;);
        tapestry.event178957379=function(e){
            tapestry.event.stopEvent(e);
            
                
                    tapestry.form.submit(&quot;searchForm&quot;, &quot;Submit&quot;, {&quot;async&quot;:true,&quot;json&quot;:false,&quot;url&quot;:&quot;/inspireapp/Blacklist,searchComponent.searchForm.sdirect?updateParts=searchComponent&quot;});
                
                
            
            
        };
        tapestry.connect(&quot;Submit&quot;, &quot;onclick&quot;, &quot;event178957379&quot;);
tapestry.cleanConnect(&quot;Submit_0&quot;, &quot;onclick&quot;, &quot;event1707222604&quot;);
        tapestry.event1707222604=function(e){
            tapestry.event.stopEvent(e);
            
                
                    tapestry.form.submit(&quot;searchForm&quot;, &quot;Submit_0&quot;, {&quot;async&quot;:true,&quot;json&quot;:false,&quot;url&quot;:&quot;/inspireapp/Blacklist,searchComponent.searchForm.sdirect?updateParts=searchComponent&quot;});
                
                
            
            
        };
        tapestry.connect(&quot;Submit_0&quot;, &quot;onclick&quot;, &quot;event1707222604&quot;);
tapestry.cleanConnect(&quot;Submit_1&quot;, &quot;onclick&quot;, &quot;event1707223370&quot;);
        tapestry.event1707223370=function(e){
            tapestry.event.stopEvent(e);
            
                
                    tapestry.form.submit(&quot;searchForm&quot;, &quot;Submit_1&quot;, {&quot;async&quot;:true,&quot;json&quot;:false,&quot;url&quot;:&quot;/inspireapp/Blacklist,searchComponent.searchForm.sdirect?updateParts=searchComponent&quot;});
                
                
            
            
        };
        tapestry.connect(&quot;Submit_1&quot;, &quot;onclick&quot;, &quot;event1707223370&quot;);
tapestry.cleanConnect(&quot;Submit_2&quot;, &quot;onclick&quot;, &quot;event1707223464&quot;);
        tapestry.event1707223464=function(e){
            tapestry.event.stopEvent(e);
            
                
                    tapestry.form.submit(&quot;searchForm&quot;, &quot;Submit_2&quot;, {&quot;async&quot;:true,&quot;json&quot;:false,&quot;url&quot;:&quot;/inspireapp/Blacklist,searchComponent.searchForm.sdirect?updateParts=searchComponent&quot;});
                
                
            
            
        };
        tapestry.connect(&quot;Submit_2&quot;, &quot;onclick&quot;, &quot;event1707223464&quot;);

tapestry.cleanConnect(&quot;sCriteria1&quot;, &quot;onchange&quot;, &quot;formEvent892686733&quot;);
                tapestry.formEvent892686733=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;Blacklist/searchComponent.sCriteria1&quot;, bcomponentid:&quot;sCriteria1&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;sCriteria1&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;sCriteria1&quot;, &quot;onchange&quot;, &quot;formEvent892686733&quot;);
tapestry.cleanConnect(&quot;comparator1&quot;, &quot;onchange&quot;, &quot;formEvent55030807&quot;);
                tapestry.formEvent55030807=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;Blacklist/searchComponent.comparator1&quot;, bcomponentid:&quot;comparator1&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;comparator1&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;comparator1&quot;, &quot;onchange&quot;, &quot;formEvent55030807&quot;);
tapestry.cleanConnect(&quot;sCriteria2&quot;, &quot;onchange&quot;, &quot;formEvent1403150071&quot;);
                tapestry.formEvent1403150071=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;Blacklist/searchComponent.sCriteria2&quot;, bcomponentid:&quot;sCriteria2&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;sCriteria2&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;sCriteria2&quot;, &quot;onchange&quot;, &quot;formEvent1403150071&quot;);
tapestry.cleanConnect(&quot;comparator2&quot;, &quot;onchange&quot;, &quot;formEvent255400393&quot;);
                tapestry.formEvent255400393=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;Blacklist/searchComponent.comparator2&quot;, bcomponentid:&quot;comparator2&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;comparator2&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;comparator2&quot;, &quot;onchange&quot;, &quot;formEvent255400393&quot;);
tapestry.cleanConnect(&quot;browseCriteria&quot;, &quot;onchange&quot;, &quot;formEvent151438188&quot;);
                tapestry.formEvent151438188=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;Blacklist/searchComponent.browseCriteria&quot;, bcomponentid:&quot;browseCriteria&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;browseCriteria&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;browseCriteria&quot;, &quot;onchange&quot;, &quot;formEvent151438188&quot;);
closeDialogComponent(&quot; , &quot;'&quot; , &quot;TinreadDialog&quot; , &quot;'&quot; , &quot;);

try {
  attachFocus(&quot; , &quot;'&quot; , &quot;field1&quot; , &quot;'&quot; , &quot;);
 }
 catch(e) {}
closeDialogComponent(&quot; , &quot;'&quot; , &quot;AssignedReports&quot; , &quot;'&quot; , &quot;);
closeDialogComponent(&quot; , &quot;'&quot; , &quot;ReportStatus&quot; , &quot;'&quot; , &quot;);
if(tapestry.fx) {
			var isResponse;
			tapestry.fx.attachAjaxStatus(function statusListener(status){
            	var fullDiv = document.getElementById(&quot; , &quot;'&quot; , &quot;ajaxLoaderDiv&quot; , &quot;'&quot; , &quot;);
            	var processDiv = document.getElementById(&quot; , &quot;'&quot; , &quot;pressingDiv&quot; , &quot;'&quot; , &quot;);
            	isResponse = status;
            	if(isResponse == false){
            		fullDiv.style.display=&quot;none&quot;;
            		processDiv.style.display=&quot;none&quot;;
            	}else{
            		fullDiv.style.display=&quot;&quot;;
            		setTimeout(
						function showProcessDiv(){
							if(isResponse==true){
								var processDiv = document.getElementById(&quot; , &quot;'&quot; , &quot;pressingDiv&quot; , &quot;'&quot; , &quot;);
								processDiv.style.display=&quot;&quot;;
							}
						}, 
						2000
					);
            	}
            }); 
		}
closeDialogComponent(&quot; , &quot;'&quot; , &quot;TinreadErrorDialog&quot; , &quot;'&quot; , &quot;);
closeDialogComponent(&quot; , &quot;'&quot; , &quot;TinreadMessageDialog&quot; , &quot;'&quot; , &quot;);});
// -->


id(&quot;orderby&quot;)&quot;))]</value>
      <webElementGuid>6ff5b0df-4b8c-4cd9-a75f-1cd54fc3bd0c</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
