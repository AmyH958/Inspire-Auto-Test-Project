<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_dojo.registerModulePath(tapestry, insp_118d36</name>
   <tag></tag>
   <elementGuidId>cc1ccfba-b7f4-4bcd-984f-785545bd8a34</elementGuidId>
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
      <webElementGuid>9d6c6b43-aca8-4345-a959-6953cdf35bab</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>leftmargin</name>
      <type>Main</type>
      <value>0</value>
      <webElementGuid>138e5ca7-6819-4369-a810-54a5485668fc</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>topmargin</name>
      <type>Main</type>
      <value>0</value>
      <webElementGuid>ab03c868-cf46-4d41-ad17-bfbb03db498f</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>rightmargin</name>
      <type>Main</type>
      <value>0</value>
      <webElementGuid>5ce201f8-5b66-499e-ae3b-4663468040e1</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>bottommargin</name>
      <type>Main</type>
      <value>0</value>
      <webElementGuid>089eae93-e0ed-4470-a922-0f3b56690887</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>bgcolor</name>
      <type>Main</type>
      <value>#ffffff</value>
      <webElementGuid>7d92e229-ba89-4f9b-b474-887fc60dad2d</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>Body</value>
      <webElementGuid>ed8b47f8-2c56-4812-a277-5ba84925e645</webElementGuid>
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
				
				最近登入:2024-03-12 15:47:21 ,成功
				
				
					
					
                       0 筆新訊息
                    
                
				
				繁體中文English繁體中文
				
English
繁體中文
						
						
				
				
					
					登 出
				
			
		
		
		
			
			
				神資圖書館(15trunk機)
			
			
		
	
	
	
	
	
		
		
		
			
			
			 查詢  編目  流通  採購   期刊控制   場地設備  說明  我的帳戶  學科服務  管理  統計  清單  Portal  臉辨報表  ERM 
			簡單查詢進階查詢FRBR查詢最近的查詢已儲存的記錄FRBR作品書目書目記錄維護註記刪除之書目記錄書目記錄全域修改權威記錄權威記錄維護註記刪除之權威記錄權威記錄全域修改條碼號館藏資料維護註記刪除之館藏資料維護出版者出版者資料維護內部移送作業書單維護FRBR作品辦證讀者記錄維護檢查讀者註記刪除之讀者記錄讀者證卡記錄維護離職離校借閱檯讀者流通檯還書箱讀者查詢交易紀錄違規讀者查詢讀者個人違規記錄指定參考書交易歷史查詢一般搜尋交易館藏預約順位調整題名預約順位調整批次修改到期日內部移送作業盤點盤點維護跨館預約/調閱跨館預約/調閱需求移轉-寄送項目移轉-收到項目盤點比對批次報廢選擇報廢執行報廢已報廢歷史查詢書箱作業條碼號空號清單(條碼機印製)訂購檢查薦購請購訂購明細訂單登收用卷期登收程序發票款項維護預算經費供應商標案內部移送作業催缺候選催缺項目催缺記錄期刊主檔訂購檢查薦購請購核可訂購明細訂單登收用卷期登收程序發票款項維護預算經費供應商裝訂裝訂主檔維護擬裝設定送裝確認裝訂登收標案內部移送作業催缺候選催缺項目催缺記錄流通記錄預約記錄借用記錄維護分類維護單項維護參數設定流通政策維護預約審核場地設備黑名單書籤訊息排程工作日誌學科館藏館藏資料類型設定學科館藏維護學科預約教師資料維護讀者評價選單學科預約維護學科預約查詢教師評語學科報表學科館藏冊數統計教師學科服務時數與使用人次統計學科服務讀者清單學科報表學科館藏冊數統計教師學科服務時數與使用人次統計學科服務讀者清單一般設定類型/使用者(館員)片語設定系統設定貨幣匯率館藏地範本設定--依館員帳號翻譯轉換 選取清單格式屬性條碼號規範顯示設定圖書館系統屬性推薦範本設定館員新增申請QR Code 設定編目書目類型設定MARC規範格式設定顯示設定停用字SDI維護同義字條碼號原則對應流通流通通知單範本流通通知單參數設定流通通知單發送紀錄日常作業排程一般每周開閉館設定閉館日期維護交易紀錄類型交易方式可適用讀者身份交易方式可適用館藏流通類別交易方式可適用使用類別交易紀錄-館藏狀態流通政策讀者證卡原則單位所系夾書單欄位設定讀者閱讀興趣發送紀錄期刊週期傳閱清單維護 轉入Z39屬性設定更新書封上傳書封索引編修索引重建功能維護公佈欄維護意見回覆維護主題維護特別書籍維護心得分享東區互惠審核東區互惠匯出系統代碼維護貨幣代碼作者號維護科特號維護OPACSDI維護讀者評論管理Help維護熱門關鍵字過濾館員Log記錄館員登入/出記錄館員資料變更記錄讀者資料變更記錄權限設定MARC權限設定功能權限設定角色設定系統狀態排程管理排程監控者排程日誌查詢統計檢索點/檢索語統計報表不同時段使用次數統計書目下載次數統計電子資源統計電子資源-館藏點閱次數統計電子資源-讀者點閱次數統計電子資源-單位系所點閱次數統計編目統計館藏量統計報表類號區間統計報表圖書增減統計報表編目/館別/大類交叉分析報表大類/冊數統計表大類/特藏交叉統計表特藏/冊數統計表新書特藏大類交叉統計圖書增減表／月份／大類大類語言種數冊數統計大類/條碼號類別冊數統計圖書冊數分類統計單圖書冊數金額分類統計非書資料數量金額統計資料類型範圍館藏統計表編目工作量統計(個人)編目工作量統計(各館)館藏狀態統計報表流通統計讀者借閱排行榜書展借閱清單流通人數統計(學制分頁 符合科系)讀者類型借閱暨歸還流通量統計報表流通人次冊數統計  利用概況統計(日報表)讀者借閱交叉分析各館特藏預約統計報表館內使用統計讀者身份借閱統計讀者個人及單位所系借閱排行榜館藏資料借閱統計單位系所借閱排行榜單位系所借閱統計熱門借閱排行榜熱門預約排行榜讀者辦証暨補發人次統計表借閱交叉分析統計資料類型流通統計館員流通量統計單位(個人)保管圖書(非書)數量明細館際互借代還統計-本館館藏館際互借代還統計-他館館藏指定參考書借閱量統計表各大類熱門借閱排行榜大專院校圖書館讀者身份借閱清單及統計報表館藏流通量統計跨館預約/調閱館藏借閱排行榜罰款收款人數/次數統計跨館預約/調閱使用量統計罰款收款統計特定館藏借閱統計採購統計書商採購統計年度經費採購統計特藏訂購方式金額統計書商到書率書商績效統計期刊統計期刊到刊率統計報表平均收刊日統計報表歷年期刊借閱統計書商到書率書商催缺次數書商累計訂購次數期刊訂費漲幅統計逾期違約金統計期刊經費訂購統計報表場地設備場地設備使用統計表報表模組範本維護編目清單書目標籤報表Tag856檢查報表新版書標維護次分類交叉分析登錄簿條碼產生清單登錄簿清單條碼號空號清單單位科系館藏設定清單館藏狀態預約清單特殊館藏清單館藏狀態清單報表流通清單罰款未收款報表預約待取清單借閱(逾期)清單還書清單新書清單讀者欠款清單借閱逾期清單罰款收款清單預約撤架清單長期借閱圖書清單離線流通清單預約額滿資料清單單位系所借閱逾期清單跨館還書清單離職離校清單跨館移送清單預約保留結束清單指定參考書借閱量明細表未外借館藏清單重複讀者比對報表借閱逾期通知單取書櫃上架清單預約圖書清單採購清單年度經費採買書籍運用率訂購資料明細表擬購資料明細表請購單詢價單訂購逾期清單謝函書的催缺報表薦購書刊複本比對報表採購參考清單財產增加清冊財產減少清冊已付款未到清單訂購明細期刊清單期刊合訂本登錄簿交贈期刊清單期刊架位清單裝訂歷史清單薦購書刊複本比對報表期刊登錄簿索贈清單期刊查驗清單已付款未到清單場地設備預約取消清單臉辨用戶使用分析表臉辨用戶與圖書分類關係表期間辦證統計_年報期間辦證統計_月報期間辦證明細期間辦證異動統計_年報期間辦證異動統計_月報辦證異動明細臉辨使用紀錄統計_年報臉辨使用紀錄統計_月報臉辨使用紀錄明細人數統計表人數統計明細表人次統計表人次統計明細表熱門時段統計表讀者進出排行榜期間辦證刪除明細系統參數設定IP區間管理延伸查詢設定系統設定檔資源瀏覽/後分類設定資源到期通知設定可使用身分類別設定 代碼管理所屬資料庫清單共用代碼檔共用代碼類別電子資源管理資源前端顯示設定Ezproxy設定檔維護電子資料庫/網路資源電子期刊/電子書電子資源批次匯入電子資源副檔匯入讀者回報資源連線異常 Meniuri.erm.9000報表點閱資源排行榜依學院別使用統計依單位別使用統計依月分資源使用統計依學院登入統計表依系所登入統計表依身分登入統計表登入清單依身分別使用統計表使用者使用資源清單 收藏資源排行榜偵測電子資源狀態設定偵測資源時間設定偵測電子資源查詢電子資源狀態電子資源狀況報表
				
	


		
			
				編目 > 內部移送作業		
			
 	



	j(function () {
		j(&quot;div[id='HiddenDiacritics']&quot;).draggable({
  	      containment: &quot;#box&quot;,
 	       scroll: false
	});
	});
	j(function () {
		j(&quot;div[id='hiddenDiacritics']&quot;).draggable({
	        containment: &quot;#box&quot;,
	        scroll: false
	});
	});
 

	function syncWithOrder() {
		document.getElementById(&quot;orderCriteria&quot;).value = document
				.getElementById(&quot;browseCriteria&quot;).value;
	}
	function syncWithBrowse() {
		document.getElementById(&quot;browseCriteria&quot;).value = document
				.getElementById(&quot;orderCriteria&quot;).value;
	}

	var refreshTime = 0;

	function apasa() {
		refreshTime = 2500;
		clickLinkSubmit(&quot;searchForm&quot;, &quot;scriptSubmit&quot;);
	}

	function showingOrderBy(status) {
		document.getElementById(&quot;showOrderBy&quot;).style.display = status;
	}
	j(document).ready(function() {
		j(&quot;#resetbutton&quot;).click(function() {
			//j(&quot;#reseter&quot;).click();
			var h = j(&quot;#reseter&quot;).attr(&quot;href&quot;);
			window.location = h;
		});

	    // JSON 資料結構模擬 start
	    var treeJsonData = JSON.parse(j('#locData').text());
	    
	    j('.place').treeoptions({
	        data: treeJsonData,
	        openImg: '/inspireapp/images/ico_add.gif', // img 路徑
	        cleanImg:'/inspireapp/images/clear.gif' // img 路徑
	    });
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
	});

	function createPopEdit(href) {

		popupwindow = window
				.open(
						&quot;&quot;,
						&quot;MeniuCatalogare&quot;,
						&quot;status=no,toolbar=0,scrollbars=yes,menubar=0,titlebar=0,resizable=1,width=1500,height=1500&quot;);
		popupwindow.moveTo(screen.width / 2 - 435, screen.height / 2 - 300);
		popupwindow.focus();

		popupwindow.location = href;

		if (popupwindow == null)
			popupwindow.opener = self;
		return false;

	}

	function createUploadPopEdit(href) {
		popupwindow = window.open(&quot;&quot;, &quot;Upload&quot;,
				&quot;status=no,toolbar=0,scrollbars=yes,menubar=0,titlebar=0,resizable=1,top=&quot;
						+ (screen.height - 150) / 2 + &quot;,left=&quot;
						+ (screen.width - 600) / 2 + &quot;,width=600,height=250&quot;);
		popupwindow.focus();

		popupwindow.location = href;

		if (popupwindow == null)
			popupwindow.opener = self;
		return false;
	}
	
	function runScript(e) {
	    if (e.keyCode == 13) {
	        document.getElementById(&quot;browse&quot;).click();
	        return false;
	    }
	}
	
	function changeModeDialog(msg, href){
		var crs = document.getElementsByClassName('search_list_c').length;
		
		if(crs){
			if(confirm(msg)){
				return createUploadPopEdit(href);
			}
			return false;
		}
		else{
			return createUploadPopEdit(href);
		}
	}



	  
		refreshTime = 0;
	
	







	











































		
			
				 
						 查詢
								
				   
						 條碼號輸入模式 
				
				  
				  上傳條碼號
				
			
		

		

		
			
				
					
						查詢條件
					
					
						
書名
出版者/書商/捐贈者
出版地
ISBN/ISSN
系統識別號
條碼號
其他號碼
索書號
分類號
館藏登收日期
館藏新增日期
館藏新增館員姓名
書目建立館員帳號
館藏修改日期
館藏修改館員姓名
館藏修改館員帳號
  
起始以
包含
等於(=)
大於(>=)
小於(&lt;=)
不等於(≠)

						 
								    
								
								
							 健康 ;健康100健康100 ;健康100 /健康101 : 101種天然食物提昇免疫力 /健康1本通 ;健康365 ;健康365 /健康4大基石 : 合理膳食, 適量運動, 戒煙限酒, 心理平衡. /健康6+1
								
								
							 
					
					
						
and
or
not
 
書名
出版者/書商/捐贈者
出版地
ISBN/ISSN
系統識別號
條碼號
其他號碼
索書號
分類號
館藏登收日期
館藏新增日期
館藏新增館員姓名
書目建立館員帳號
館藏修改日期
館藏修改館員姓名
館藏修改館員帳號
  
起始以
包含
等於(=)
大於(>=)
小於(&lt;=)
不等於(≠)

						 
								    
								
								
							 
								
								
							 
					
					
						
and
or
not
 
書名
出版者/書商/捐贈者
出版地
ISBN/ISSN
系統識別號
條碼號
其他號碼
索書號
分類號
館藏登收日期
館藏新增日期
館藏新增館員姓名
書目建立館員帳號
館藏修改日期
館藏修改館員姓名
館藏修改館員帳號
  
起始以
包含
等於(=)
大於(>=)
小於(&lt;=)
不等於(≠)

						  
								    
								
								
							
						
								
								
							
						
					
					
						
and
or
not
 
書名
出版者/書商/捐贈者
出版地
ISBN/ISSN
系統識別號
條碼號
其他號碼
索書號
分類號
館藏登收日期
館藏新增日期
館藏新增館員姓名
書目建立館員帳號
館藏修改日期
館藏修改館員姓名
館藏修改館員帳號
  
起始以
包含
等於(=)
大於(>=)
小於(&lt;=)
不等於(≠)

						  
								    
								
								
							
						
								
								
							
						
					

					
						
							 限制條件
						  
						
						
					
				
				
					
						
					
					
						
						
					
				
			

			
				
			

			
				
					{&quot;data&quot; :[{&quot;name&quot; : &quot;BD - 藍光光碟&quot;},{&quot;name&quot; : &quot;CA - 靜畫資料&quot;},{&quot;name&quot; : &quot;DB - 資料庫&quot;},{&quot;name&quot; : &quot;DF - 磁片&quot;},{&quot;name&quot; : &quot;DO - 電子書&quot;},{&quot;name&quot; : &quot;EA - 立體模型&quot;},{&quot;name&quot; : &quot;EB - 線上電子書&quot;},{&quot;name&quot; : &quot;EJ - 線上電子期刊&quot;},{&quot;name&quot; : &quot;EP - 電子期刊光碟&quot;},{&quot;name&quot; : &quot;ERROR - 有問題特藏&quot;},{&quot;name&quot; : &quot;FA - 磁帶&quot;},{&quot;name&quot; : &quot;KT - 多媒體組件&quot;},{&quot;name&quot; : &quot;LA - 地圖&quot;},{&quot;name&quot; : &quot;LD - 影碟&quot;},{&quot;name&quot; : &quot;MP - MP3&quot;},{&quot;name&quot; : &quot;NH - 微縮單片&quot;},{&quot;name&quot; : &quot;NR - 微縮捲片&quot;},{&quot;name&quot; : &quot;QA - 地球儀&quot;},{&quot;name&quot; : &quot;R - 參考書&quot;},{&quot;name&quot; : &quot;SL - 幻燈片&quot;},{&quot;name&quot; : &quot;VC - 錄影帶&quot;},{&quot;name&quot; : &quot;VD - VCD&quot;},{&quot;name&quot; : &quot;BOX - 書箱&quot;},{&quot;name&quot; : &quot;ERM_DB - 電子資料庫(ERM)&quot;},{&quot;name&quot; : &quot;ERM_WS - 網路資源(ERM)&quot;},{&quot;name&quot; : &quot;ERM_EB - 電子書(ERM)&quot;},{&quot;name&quot; : &quot;ERM_EJ - 電子期刊(ERM)&quot;},{&quot;name&quot; : &quot;XL - X-ray&quot;},{&quot;name&quot; : &quot;BOOK - 圖書&quot;},{&quot;name&quot; : &quot;AC - 錄音帶&quot;},{&quot;name&quot; : &quot;APP - 附件&quot;},{&quot;name&quot; : &quot;P - 現期期刊&quot;},{&quot;name&quot; : &quot;ac - ac&quot;},{&quot;name&quot; : &quot;DD - 影像光碟(DVD)&quot;},{&quot;name&quot; : &quot;MD - 行動設備&quot;},{&quot;name&quot; : &quot;S - 裝訂期刊&quot;},{&quot;name&quot; : &quot;AD - 唱片&quot;},{&quot;name&quot; : &quot;CD - 光碟&quot;},{&quot;name&quot; : &quot;booklet - 小冊子&quot;},{&quot;name&quot; : &quot;KKtest - KK&quot;},{&quot;name&quot; : &quot;YYtest2 - YY2&quot;},{&quot;name&quot; : &quot;YYtest4 - YYYtest&quot;},{&quot;name&quot; : &quot;0425 - 0425&quot;},{&quot;name&quot; : &quot;TEST - TEST&quot;},{&quot;name&quot; : &quot;TEST0425 - TEST0425&quot;}]}{&quot;data&quot; :[{&quot;name&quot; : &quot;B可借圖書&quot;},{&quot;name&quot; : &quot;B電子資源&quot;},{&quot;name&quot; : &quot;eee&quot;},{&quot;name&quot; : &quot;M可借行動設備&quot;},{&quot;name&quot; : &quot;P可借期刊&quot;},{&quot;name&quot; : &quot;V可借視聽&quot;},{&quot;name&quot; : &quot;www&quot;},{&quot;name&quot; : &quot;不流通&quot;},{&quot;name&quot; : &quot;書箱借閱30天&quot;}]}
						
							限制條件
							
						
						
							書目性質:
							
全系統記錄
圖書
期刊
分析款目
合集

						

						
						
						
						
						
							資料類型:
							
					   		
						


						
							館藏地:
							
								
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




								
						

						

						

						

						
						
						
						
							
							館藏狀態:
							
							
							

在架
借出
預約保留
遺失
盤點未到
聲明歸還
採購處理中
移送編目
編目處理中
移送閱覽
流通處理中
此筆盤點
核准跨館移送
等待送回原館藏地
等待轉送其他館藏地
跨館移送中
跨館轉送中
送回原館藏地
報銷中
報銷找回
重新歸架
展示中
已送裝訂
暫時不可提供
待報廢
已報廢
書箱借出
久借未還
尋書未獲
PickListValues.15004108
盤點結果狀態測試
測試選項
CWEN測試

							
						
						
						
							 
							 	館藏流通類別:
							 
							
							
							
						

						
							

							

							

							
							
						
					

				
			

		


		
			
				
					請指定要修改的資訊：
				
					
						館藏狀態
					

在架
盤點未到
採購處理中
移送編目
編目處理中
移送閱覽
流通處理中
此筆盤點
報銷中
報銷找回
重新歸架
展示中
暫時不可提供
久借未還
尋書未獲
PickListValues.15004108
盤點結果狀態測試
測試選項
CWEN測試

				
				
					
						在OPAC顯示
					

不在OPAC上顯示
在OPAC上顯示

					
				
				
					
						館藏地
					
					
							 
	function runScript(e) {
	    if (e.keyCode == 13) {
	        document.getElementById(&quot;browse&quot;).click();
	        return false;
	    }
	}				

 
 
 
  
  
   
  
  
  
  
   
  
  
   
    
   
   
館藏地CMUL - 神資圖書館2 - 2123 - 12320230417 - 2023041720230418 - 20230418AH - 安南醫院AHGL - 安南圖書區av - avB007 - B007BCSB4 - BCSB4BX - 取書櫃1BY - 取書櫃2CB - 北港分館BAVN - 北港分館視聽區(限館內閱覽)BCIR - 北港分館流通櫃檯BCRA - 北港分館指參(限館內閱覽)BPAV - 北港分館視聽區BPCL - 北港分館書庫cbook - cbookcircd - circdclp - clpCM - 北港附設醫院BMHL - 北港附設醫院圖書室CMUL - 中國醫藥大學圖書館 - CMUL - 中國醫藥大學圖書館CU - 台中總館CUAV - 台中總館視聽區(獨立專區)MAVN - 台中總館視聽區(限館內閱覽)MAVR - 台中總館視聽區MCAT - 台中總館技服組MCBS - 台中總館密閉書庫MCIR - 台中總館流通櫃檯MCRA - 台中總館教師指定參考書(限館內閱覽)MCSS - 台中總館B3裝訂期刊區MDIA - 台中總館博碩士論文區MEAS - 台中總館探索史懷哲之路專書區MEXM - 台中總館國考書區(限館內閱覽)MFGA - 台中總館本校教職優良教材區MFPA - 台中總館本校教師升等資料區MFSA - 台中總館本校教職論著MHME - 台中總館人文專書區-醫學倫理(限館內閱覽)台中總館人文專書區-醫學倫理(限館內閱覽)MHMH - 台中總館人文專書區-醫療史(限館內閱覽)MHML - 台中總館人文專書區-醫學法律(限館內閱覽)MHQC - 台中總館人文專書區-醫療品質(限館內閱覽)MNBR - 台中總館新書展示區MPAA - 台中總館績效暨獲獎區(限館內閱覽)MPBL - 台中總館PBL專書區(限館內閱覽)MREF - 台中總館參考室MRSS - 台中總館閱覽組MS-C - 台中總館期刊複本櫃MSER - 台中總館期刊區MSPA - 台中總館研究計劃專書MSPB - 中醫醫史文獻室(限所內閱覽)MSPC - 台中總館特藏室MSTK - 台中總館書庫MYBK - 台中總館參考壁櫃new item 7 - new item 7ONLN - 台中總館線上資料e-resources - 電子資源EB-P - EB-Pelect - electH-EQ - H-EQH-MR - H-MRL - LL40 - L40LB 圖書總館 - LB 圖書總館LB-S - LB-SLE - 語文教學中心LEGL - 語文教學中心圖書室LIB - LIBnew item 1 - new item 1new item 3 - new item 3new item 10 - new item 10new item 12 - new item 12new item 13 - new item 13new item 14 - new item 14new item 16 - new item 16new item 2 - new item 2new item 20 - new item 20new item 4 - new item 4new item 5 - new item 5new item 6 - 英才校區new item 8 - new item 8new item 9 - new item 9NPTU - NPTUOUK - OUKPT - 培德醫院new item 11 - new item 11new item 17 - new item 17PTGL - 培德醫院圖書區ptext - ptextSB3 - SB3T-P - T-PTBBK - TBBKTH - 台北分院THGL - 台北分院圖書區THPA - 台北分院期刊區W-P - W-PYH - 豐原分院new item 18 - new item 18new item 19 - new item 19YHGL - 豐原分院圖書區z3llc - z3llcz6bkf - z6bkfzd1a2 - zd1a2zd1e - zd1ezdlf - zdlf五樓漫畫書專區 - 五樓漫畫書專區實體館藏 - 實體館藏綜合書庫 - 綜合書庫艾迪訊圖書館 - 艾迪訊圖書館附中出版物專區 - 附中出版物專區龍華科技大學圖書館 - 龍華科技大學圖書館
  
  
window.dlocation = new dTree('window.dlocation', 'messages', '/inspireapp/images/'); 
window.dlocation.add(0,-1,'館藏地'); 
window.dlocation.add(1,0,&quot;CMUL - \u795E\u8CC7\u5716\u66F8\u9928&quot;, &quot;javascript:window.dlocation.selectElement('CMUL - \\u795E\\u8CC7\\u5716\\u66F8\\u9928', 1, true)&quot;); 
window.dlocation.add(463,1,&quot;2 - 2&quot;, &quot;javascript:window.dlocation.selectElement('2 - 2', 463, true)&quot;); 
window.dlocation.add(583,1,&quot;123 - 123&quot;, &quot;javascript:window.dlocation.selectElement('123 - 123', 583, true)&quot;); 
window.dlocation.add(1183,1,&quot;20230417 - 20230417&quot;, &quot;javascript:window.dlocation.selectElement('20230417 - 20230417', 1183, true)&quot;); 
window.dlocation.add(1203,1,&quot;20230418 - 20230418&quot;, &quot;javascript:window.dlocation.selectElement('20230418 - 20230418', 1203, true)&quot;); 
window.dlocation.add(167,1,&quot;AH - \u5B89\u5357\u91AB\u9662&quot;, &quot;javascript:window.dlocation.selectElement('AH - \\u5B89\\u5357\\u91AB\\u9662', 167, true)&quot;); 
window.dlocation.add(177,167,&quot;AHGL - \u5B89\u5357\u5716\u66F8\u5340&quot;, &quot;javascript:window.dlocation.selectElement('AHGL - \\u5B89\\u5357\\u5716\\u66F8\\u5340', 177, true)&quot;); 
window.dlocation.add(643,1,&quot;av - av&quot;, &quot;javascript:window.dlocation.selectElement('av - av', 643, true)&quot;); 
window.dlocation.add(303,1,&quot;B007 - B007&quot;, &quot;javascript:window.dlocation.selectElement('B007 - B007', 303, true)&quot;); 
window.dlocation.add(883,1,&quot;BCSB4 - BCSB4&quot;, &quot;javascript:window.dlocation.selectElement('BCSB4 - BCSB4', 883, true)&quot;); 
window.dlocation.add(823,1,&quot;BX - \u53D6\u66F8\u6AC31&quot;, &quot;javascript:window.dlocation.selectElement('BX - \\u53D6\\u66F8\\u6AC31', 823, true)&quot;); 
window.dlocation.add(903,1,&quot;BY - \u53D6\u66F8\u6AC32&quot;, &quot;javascript:window.dlocation.selectElement('BY - \\u53D6\\u66F8\\u6AC32', 903, true)&quot;); 
window.dlocation.add(169,1,&quot;CB - \u5317\u6E2F\u5206\u9928&quot;, &quot;javascript:window.dlocation.selectElement('CB - \\u5317\\u6E2F\\u5206\\u9928', 169, true)&quot;); 
window.dlocation.add(179,169,&quot;BAVN - \u5317\u6E2F\u5206\u9928\u8996\u807D\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dlocation.selectElement('BAVN - \\u5317\\u6E2F\\u5206\\u9928\\u8996\\u807D\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 179, true)&quot;); 
window.dlocation.add(180,169,&quot;BCIR - \u5317\u6E2F\u5206\u9928\u6D41\u901A\u6AC3\u6AAF&quot;, &quot;javascript:window.dlocation.selectElement('BCIR - \\u5317\\u6E2F\\u5206\\u9928\\u6D41\\u901A\\u6AC3\\u6AAF', 180, true)&quot;); 
window.dlocation.add(181,169,&quot;BCRA - \u5317\u6E2F\u5206\u9928\u6307\u53C3(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dlocation.selectElement('BCRA - \\u5317\\u6E2F\\u5206\\u9928\\u6307\\u53C3(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 181, true)&quot;); 
window.dlocation.add(182,169,&quot;BPAV - \u5317\u6E2F\u5206\u9928\u8996\u807D\u5340&quot;, &quot;javascript:window.dlocation.selectElement('BPAV - \\u5317\\u6E2F\\u5206\\u9928\\u8996\\u807D\\u5340', 182, true)&quot;); 
window.dlocation.add(183,169,&quot;BPCL - \u5317\u6E2F\u5206\u9928\u66F8\u5EAB&quot;, &quot;javascript:window.dlocation.selectElement('BPCL - \\u5317\\u6E2F\\u5206\\u9928\\u66F8\\u5EAB', 183, true)&quot;); 
window.dlocation.add(623,1,&quot;cbook - cbook&quot;, &quot;javascript:window.dlocation.selectElement('cbook - cbook', 623, true)&quot;); 
window.dlocation.add(624,1,&quot;circd - circd&quot;, &quot;javascript:window.dlocation.selectElement('circd - circd', 624, true)&quot;); 
window.dlocation.add(683,1,&quot;clp - clp&quot;, &quot;javascript:window.dlocation.selectElement('clp - clp', 683, true)&quot;); 
window.dlocation.add(170,1,&quot;CM - \u5317\u6E2F\u9644\u8A2D\u91AB\u9662&quot;, &quot;javascript:window.dlocation.selectElement('CM - \\u5317\\u6E2F\\u9644\\u8A2D\\u91AB\\u9662', 170, true)&quot;); 
window.dlocation.add(211,170,&quot;BMHL - \u5317\u6E2F\u9644\u8A2D\u91AB\u9662\u5716\u66F8\u5BA4&quot;, &quot;javascript:window.dlocation.selectElement('BMHL - \\u5317\\u6E2F\\u9644\\u8A2D\\u91AB\\u9662\\u5716\\u66F8\\u5BA4', 211, true)&quot;); 
window.dlocation.add(363,1,&quot;CMUL - \u4E2D\u570B\u91AB\u85E5\u5927\u5B78\u5716\u66F8\u9928 - CMUL - \u4E2D\u570B\u91AB\u85E5\u5927\u5B78\u5716\u66F8\u9928&quot;, &quot;javascript:window.dlocation.selectElement('CMUL - \\u4E2D\\u570B\\u91AB\\u85E5\\u5927\\u5B78\\u5716\\u66F8\\u9928 - CMUL - \\u4E2D\\u570B\\u91AB\\u85E5\\u5927\\u5B78\\u5716\\u66F8\\u9928', 363, true)&quot;); 
window.dlocation.add(171,1,&quot;CU - \u53F0\u4E2D\u7E3D\u9928&quot;, &quot;javascript:window.dlocation.selectElement('CU - \\u53F0\\u4E2D\\u7E3D\\u9928', 171, true)&quot;); 
window.dlocation.add(603,171,&quot;CUAV - \u53F0\u4E2D\u7E3D\u9928\u8996\u807D\u5340(\u7368\u7ACB\u5C08\u5340)&quot;, &quot;javascript:window.dlocation.selectElement('CUAV - \\u53F0\\u4E2D\\u7E3D\\u9928\\u8996\\u807D\\u5340(\\u7368\\u7ACB\\u5C08\\u5340)', 603, true)&quot;); 
window.dlocation.add(217,171,&quot;MAVN - \u53F0\u4E2D\u7E3D\u9928\u8996\u807D\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dlocation.selectElement('MAVN - \\u53F0\\u4E2D\\u7E3D\\u9928\\u8996\\u807D\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 217, true)&quot;); 
window.dlocation.add(218,171,&quot;MAVR - \u53F0\u4E2D\u7E3D\u9928\u8996\u807D\u5340&quot;, &quot;javascript:window.dlocation.selectElement('MAVR - \\u53F0\\u4E2D\\u7E3D\\u9928\\u8996\\u807D\\u5340', 218, true)&quot;); 
window.dlocation.add(219,171,&quot;MCAT - \u53F0\u4E2D\u7E3D\u9928\u6280\u670D\u7D44&quot;, &quot;javascript:window.dlocation.selectElement('MCAT - \\u53F0\\u4E2D\\u7E3D\\u9928\\u6280\\u670D\\u7D44', 219, true)&quot;); 
window.dlocation.add(220,171,&quot;MCBS - \u53F0\u4E2D\u7E3D\u9928\u5BC6\u9589\u66F8\u5EAB&quot;, &quot;javascript:window.dlocation.selectElement('MCBS - \\u53F0\\u4E2D\\u7E3D\\u9928\\u5BC6\\u9589\\u66F8\\u5EAB', 220, true)&quot;); 
window.dlocation.add(221,171,&quot;MCIR - \u53F0\u4E2D\u7E3D\u9928\u6D41\u901A\u6AC3\u6AAF&quot;, &quot;javascript:window.dlocation.selectElement('MCIR - \\u53F0\\u4E2D\\u7E3D\\u9928\\u6D41\\u901A\\u6AC3\\u6AAF', 221, true)&quot;); 
window.dlocation.add(244,171,&quot;MCRA - \u53F0\u4E2D\u7E3D\u9928\u6559\u5E2B\u6307\u5B9A\u53C3\u8003\u66F8(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dlocation.selectElement('MCRA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u6559\\u5E2B\\u6307\\u5B9A\\u53C3\\u8003\\u66F8(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 244, true)&quot;); 
window.dlocation.add(245,171,&quot;MCSS - \u53F0\u4E2D\u7E3D\u9928B3\u88DD\u8A02\u671F\u520A\u5340&quot;, &quot;javascript:window.dlocation.selectElement('MCSS - \\u53F0\\u4E2D\\u7E3D\\u9928B3\\u88DD\\u8A02\\u671F\\u520A\\u5340', 245, true)&quot;); 
window.dlocation.add(246,171,&quot;MDIA - \u53F0\u4E2D\u7E3D\u9928\u535A\u78A9\u58EB\u8AD6\u6587\u5340&quot;, &quot;javascript:window.dlocation.selectElement('MDIA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u535A\\u78A9\\u58EB\\u8AD6\\u6587\\u5340', 246, true)&quot;); 
window.dlocation.add(248,171,&quot;MEAS - \u53F0\u4E2D\u7E3D\u9928\u63A2\u7D22\u53F2\u61F7\u54F2\u4E4B\u8DEF\u5C08\u66F8\u5340&quot;, &quot;javascript:window.dlocation.selectElement('MEAS - \\u53F0\\u4E2D\\u7E3D\\u9928\\u63A2\\u7D22\\u53F2\\u61F7\\u54F2\\u4E4B\\u8DEF\\u5C08\\u66F8\\u5340', 248, true)&quot;); 
window.dlocation.add(250,171,&quot;MEXM - \u53F0\u4E2D\u7E3D\u9928\u570B\u8003\u66F8\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dlocation.selectElement('MEXM - \\u53F0\\u4E2D\\u7E3D\\u9928\\u570B\\u8003\\u66F8\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 250, true)&quot;); 
window.dlocation.add(251,171,&quot;MFGA - \u53F0\u4E2D\u7E3D\u9928\u672C\u6821\u6559\u8077\u512A\u826F\u6559\u6750\u5340&quot;, &quot;javascript:window.dlocation.selectElement('MFGA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u672C\\u6821\\u6559\\u8077\\u512A\\u826F\\u6559\\u6750\\u5340', 251, true)&quot;); 
window.dlocation.add(252,171,&quot;MFPA - \u53F0\u4E2D\u7E3D\u9928\u672C\u6821\u6559\u5E2B\u5347\u7B49\u8CC7\u6599\u5340&quot;, &quot;javascript:window.dlocation.selectElement('MFPA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u672C\\u6821\\u6559\\u5E2B\\u5347\\u7B49\\u8CC7\\u6599\\u5340', 252, true)&quot;); 
window.dlocation.add(253,171,&quot;MFSA - \u53F0\u4E2D\u7E3D\u9928\u672C\u6821\u6559\u8077\u8AD6\u8457&quot;, &quot;javascript:window.dlocation.selectElement('MFSA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u672C\\u6821\\u6559\\u8077\\u8AD6\\u8457', 253, true)&quot;); 
window.dlocation.add(256,171,&quot;MHME - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u5B78\u502B\u7406(\u9650\u9928\u5167\u95B1\u89BD)\u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u5B78\u502B\u7406(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dlocation.selectElement('MHME - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u5B78\\u502B\\u7406(\\u9650\\u9928\\u5167\\u95B1\\u89BD)\\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u5B78\\u502B\\u7406(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 256, true)&quot;); 
window.dlocation.add(257,171,&quot;MHMH - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u7642\u53F2(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dlocation.selectElement('MHMH - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u7642\\u53F2(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 257, true)&quot;); 
window.dlocation.add(258,171,&quot;MHML - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u5B78\u6CD5\u5F8B(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dlocation.selectElement('MHML - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u5B78\\u6CD5\\u5F8B(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 258, true)&quot;); 
window.dlocation.add(259,171,&quot;MHQC - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u7642\u54C1\u8CEA(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dlocation.selectElement('MHQC - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u7642\\u54C1\\u8CEA(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 259, true)&quot;); 
window.dlocation.add(261,171,&quot;MNBR - \u53F0\u4E2D\u7E3D\u9928\u65B0\u66F8\u5C55\u793A\u5340&quot;, &quot;javascript:window.dlocation.selectElement('MNBR - \\u53F0\\u4E2D\\u7E3D\\u9928\\u65B0\\u66F8\\u5C55\\u793A\\u5340', 261, true)&quot;); 
window.dlocation.add(262,171,&quot;MPAA - \u53F0\u4E2D\u7E3D\u9928\u7E3E\u6548\u66A8\u7372\u734E\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dlocation.selectElement('MPAA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7E3E\\u6548\\u66A8\\u7372\\u734E\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 262, true)&quot;); 
window.dlocation.add(263,171,&quot;MPBL - \u53F0\u4E2D\u7E3D\u9928PBL\u5C08\u66F8\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dlocation.selectElement('MPBL - \\u53F0\\u4E2D\\u7E3D\\u9928PBL\\u5C08\\u66F8\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)', 263, true)&quot;); 
window.dlocation.add(265,171,&quot;MREF - \u53F0\u4E2D\u7E3D\u9928\u53C3\u8003\u5BA4&quot;, &quot;javascript:window.dlocation.selectElement('MREF - \\u53F0\\u4E2D\\u7E3D\\u9928\\u53C3\\u8003\\u5BA4', 265, true)&quot;); 
window.dlocation.add(266,171,&quot;MRSS - \u53F0\u4E2D\u7E3D\u9928\u95B1\u89BD\u7D44&quot;, &quot;javascript:window.dlocation.selectElement('MRSS - \\u53F0\\u4E2D\\u7E3D\\u9928\\u95B1\\u89BD\\u7D44', 266, true)&quot;); 
window.dlocation.add(267,171,&quot;MS-C - \u53F0\u4E2D\u7E3D\u9928\u671F\u520A\u8907\u672C\u6AC3&quot;, &quot;javascript:window.dlocation.selectElement('MS-C - \\u53F0\\u4E2D\\u7E3D\\u9928\\u671F\\u520A\\u8907\\u672C\\u6AC3', 267, true)&quot;); 
window.dlocation.add(270,171,&quot;MSER - \u53F0\u4E2D\u7E3D\u9928\u671F\u520A\u5340&quot;, &quot;javascript:window.dlocation.selectElement('MSER - \\u53F0\\u4E2D\\u7E3D\\u9928\\u671F\\u520A\\u5340', 270, true)&quot;); 
window.dlocation.add(271,171,&quot;MSPA - \u53F0\u4E2D\u7E3D\u9928\u7814\u7A76\u8A08\u5283\u5C08\u66F8&quot;, &quot;javascript:window.dlocation.selectElement('MSPA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7814\\u7A76\\u8A08\\u5283\\u5C08\\u66F8', 271, true)&quot;); 
window.dlocation.add(272,171,&quot;MSPB - \u4E2D\u91AB\u91AB\u53F2\u6587\u737B\u5BA4(\u9650\u6240\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dlocation.selectElement('MSPB - \\u4E2D\\u91AB\\u91AB\\u53F2\\u6587\\u737B\\u5BA4(\\u9650\\u6240\\u5167\\u95B1\\u89BD)', 272, true)&quot;); 
window.dlocation.add(273,171,&quot;MSPC - \u53F0\u4E2D\u7E3D\u9928\u7279\u85CF\u5BA4&quot;, &quot;javascript:window.dlocation.selectElement('MSPC - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7279\\u85CF\\u5BA4', 273, true)&quot;); 
window.dlocation.add(274,171,&quot;MSTK - \u53F0\u4E2D\u7E3D\u9928\u66F8\u5EAB&quot;, &quot;javascript:window.dlocation.selectElement('MSTK - \\u53F0\\u4E2D\\u7E3D\\u9928\\u66F8\\u5EAB', 274, true)&quot;); 
window.dlocation.add(275,171,&quot;MYBK - \u53F0\u4E2D\u7E3D\u9928\u53C3\u8003\u58C1\u6AC3&quot;, &quot;javascript:window.dlocation.selectElement('MYBK - \\u53F0\\u4E2D\\u7E3D\\u9928\\u53C3\\u8003\\u58C1\\u6AC3', 275, true)&quot;); 
window.dlocation.add(1103,171,&quot;new item 7 - new item 7&quot;, &quot;javascript:window.dlocation.selectElement('new item 7 - new item 7', 1103, true)&quot;); 
window.dlocation.add(276,171,&quot;ONLN - \u53F0\u4E2D\u7E3D\u9928\u7DDA\u4E0A\u8CC7\u6599&quot;, &quot;javascript:window.dlocation.selectElement('ONLN - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7DDA\\u4E0A\\u8CC7\\u6599', 276, true)&quot;); 
window.dlocation.add(3,1,&quot;e-resources - \u96FB\u5B50\u8CC7\u6E90&quot;, &quot;javascript:window.dlocation.selectElement('e-resources - \\u96FB\\u5B50\\u8CC7\\u6E90', 3, true)&quot;); 
window.dlocation.add(345,1,&quot;EB-P - EB-P&quot;, &quot;javascript:window.dlocation.selectElement('EB-P - EB-P', 345, true)&quot;); 
window.dlocation.add(648,1,&quot;elect - elect&quot;, &quot;javascript:window.dlocation.selectElement('elect - elect', 648, true)&quot;); 
window.dlocation.add(343,1,&quot;H-EQ - H-EQ&quot;, &quot;javascript:window.dlocation.selectElement('H-EQ - H-EQ', 343, true)&quot;); 
window.dlocation.add(344,1,&quot;H-MR - H-MR&quot;, &quot;javascript:window.dlocation.selectElement('H-MR - H-MR', 344, true)&quot;); 
window.dlocation.add(543,1,&quot;L - L&quot;, &quot;javascript:window.dlocation.selectElement('L - L', 543, true)&quot;); 
window.dlocation.add(863,1,&quot;L40 - L40&quot;, &quot;javascript:window.dlocation.selectElement('L40 - L40', 863, true)&quot;); 
window.dlocation.add(1023,1,&quot;LB \u5716\u66F8\u7E3D\u9928 - LB \u5716\u66F8\u7E3D\u9928&quot;, &quot;javascript:window.dlocation.selectElement('LB \\u5716\\u66F8\\u7E3D\\u9928 - LB \\u5716\\u66F8\\u7E3D\\u9928', 1023, true)&quot;); 
window.dlocation.add(323,1,&quot;LB-S - LB-S&quot;, &quot;javascript:window.dlocation.selectElement('LB-S - LB-S', 323, true)&quot;); 
window.dlocation.add(173,1,&quot;LE - \u8A9E\u6587\u6559\u5B78\u4E2D\u5FC3&quot;, &quot;javascript:window.dlocation.selectElement('LE - \\u8A9E\\u6587\\u6559\\u5B78\\u4E2D\\u5FC3', 173, true)&quot;); 
window.dlocation.add(280,173,&quot;LEGL - \u8A9E\u6587\u6559\u5B78\u4E2D\u5FC3\u5716\u66F8\u5BA4&quot;, &quot;javascript:window.dlocation.selectElement('LEGL - \\u8A9E\\u6587\\u6559\\u5B78\\u4E2D\\u5FC3\\u5716\\u66F8\\u5BA4', 280, true)&quot;); 
window.dlocation.add(523,1,&quot;LIB - LIB&quot;, &quot;javascript:window.dlocation.selectElement('LIB - LIB', 523, true)&quot;); 
window.dlocation.add(423,1,&quot;new item 1 - new item 1&quot;, &quot;javascript:window.dlocation.selectElement('new item 1 - new item 1', 423, true)&quot;); 
window.dlocation.add(484,423,&quot;new item 3 - new item 3&quot;, &quot;javascript:window.dlocation.selectElement('new item 3 - new item 3', 484, true)&quot;); 
window.dlocation.add(1283,1,&quot;new item 10 - new item 10&quot;, &quot;javascript:window.dlocation.selectElement('new item 10 - new item 10', 1283, true)&quot;); 
window.dlocation.add(1323,1,&quot;new item 12 - new item 12&quot;, &quot;javascript:window.dlocation.selectElement('new item 12 - new item 12', 1323, true)&quot;); 
window.dlocation.add(1343,1,&quot;new item 13 - new item 13&quot;, &quot;javascript:window.dlocation.selectElement('new item 13 - new item 13', 1343, true)&quot;); 
window.dlocation.add(1344,1,&quot;new item 14 - new item 14&quot;, &quot;javascript:window.dlocation.selectElement('new item 14 - new item 14', 1344, true)&quot;); 
window.dlocation.add(1264,1,&quot;new item 16 - new item 16&quot;, &quot;javascript:window.dlocation.selectElement('new item 16 - new item 16', 1264, true)&quot;); 
window.dlocation.add(483,1,&quot;new item 2 - new item 2&quot;, &quot;javascript:window.dlocation.selectElement('new item 2 - new item 2', 483, true)&quot;); 
window.dlocation.add(1425,1,&quot;new item 20 - new item 20&quot;, &quot;javascript:window.dlocation.selectElement('new item 20 - new item 20', 1425, true)&quot;); 
window.dlocation.add(943,1,&quot;new item 4 - new item 4&quot;, &quot;javascript:window.dlocation.selectElement('new item 4 - new item 4', 943, true)&quot;); 
window.dlocation.add(963,1,&quot;new item 5 - new item 5&quot;, &quot;javascript:window.dlocation.selectElement('new item 5 - new item 5', 963, true)&quot;); 
window.dlocation.add(1063,1,&quot;new item 6 - \u82F1\u624D\u6821\u5340&quot;, &quot;javascript:window.dlocation.selectElement('new item 6 - \\u82F1\\u624D\\u6821\\u5340', 1063, true)&quot;); 
window.dlocation.add(1243,1,&quot;new item 8 - new item 8&quot;, &quot;javascript:window.dlocation.selectElement('new item 8 - new item 8', 1243, true)&quot;); 
window.dlocation.add(1263,1,&quot;new item 9 - new item 9&quot;, &quot;javascript:window.dlocation.selectElement('new item 9 - new item 9', 1263, true)&quot;); 
window.dlocation.add(1043,1,&quot;NPTU - NPTU&quot;, &quot;javascript:window.dlocation.selectElement('NPTU - NPTU', 1043, true)&quot;); 
window.dlocation.add(503,1,&quot;OUK - OUK&quot;, &quot;javascript:window.dlocation.selectElement('OUK - OUK', 503, true)&quot;); 
window.dlocation.add(174,1,&quot;PT - \u57F9\u5FB7\u91AB\u9662&quot;, &quot;javascript:window.dlocation.selectElement('PT - \\u57F9\\u5FB7\\u91AB\\u9662', 174, true)&quot;); 
window.dlocation.add(1303,174,&quot;new item 11 - new item 11&quot;, &quot;javascript:window.dlocation.selectElement('new item 11 - new item 11', 1303, true)&quot;); 
window.dlocation.add(1363,174,&quot;new item 17 - new item 17&quot;, &quot;javascript:window.dlocation.selectElement('new item 17 - new item 17', 1363, true)&quot;); 
window.dlocation.add(283,174,&quot;PTGL - \u57F9\u5FB7\u91AB\u9662\u5716\u66F8\u5340&quot;, &quot;javascript:window.dlocation.selectElement('PTGL - \\u57F9\\u5FB7\\u91AB\\u9662\\u5716\\u66F8\\u5340', 283, true)&quot;); 
window.dlocation.add(645,1,&quot;ptext - ptext&quot;, &quot;javascript:window.dlocation.selectElement('ptext - ptext', 645, true)&quot;); 
window.dlocation.add(1083,1,&quot;SB3 - SB3&quot;, &quot;javascript:window.dlocation.selectElement('SB3 - SB3', 1083, true)&quot;); 
window.dlocation.add(324,1,&quot;T-P - T-P&quot;, &quot;javascript:window.dlocation.selectElement('T-P - T-P', 324, true)&quot;); 
window.dlocation.add(1403,1,&quot;TBBK - TBBK&quot;, &quot;javascript:window.dlocation.selectElement('TBBK - TBBK', 1403, true)&quot;); 
window.dlocation.add(175,1,&quot;TH - \u53F0\u5317\u5206\u9662&quot;, &quot;javascript:window.dlocation.selectElement('TH - \\u53F0\\u5317\\u5206\\u9662', 175, true)&quot;); 
window.dlocation.add(284,175,&quot;THGL - \u53F0\u5317\u5206\u9662\u5716\u66F8\u5340&quot;, &quot;javascript:window.dlocation.selectElement('THGL - \\u53F0\\u5317\\u5206\\u9662\\u5716\\u66F8\\u5340', 284, true)&quot;); 
window.dlocation.add(285,175,&quot;THPA - \u53F0\u5317\u5206\u9662\u671F\u520A\u5340&quot;, &quot;javascript:window.dlocation.selectElement('THPA - \\u53F0\\u5317\\u5206\\u9662\\u671F\\u520A\\u5340', 285, true)&quot;); 
window.dlocation.add(325,1,&quot;W-P - W-P&quot;, &quot;javascript:window.dlocation.selectElement('W-P - W-P', 325, true)&quot;); 
window.dlocation.add(176,1,&quot;YH - \u8C50\u539F\u5206\u9662&quot;, &quot;javascript:window.dlocation.selectElement('YH - \\u8C50\\u539F\\u5206\\u9662', 176, true)&quot;); 
window.dlocation.add(1423,176,&quot;new item 18 - new item 18&quot;, &quot;javascript:window.dlocation.selectElement('new item 18 - new item 18', 1423, true)&quot;); 
window.dlocation.add(1424,176,&quot;new item 19 - new item 19&quot;, &quot;javascript:window.dlocation.selectElement('new item 19 - new item 19', 1424, true)&quot;); 
window.dlocation.add(286,176,&quot;YHGL - \u8C50\u539F\u5206\u9662\u5716\u66F8\u5340&quot;, &quot;javascript:window.dlocation.selectElement('YHGL - \\u8C50\\u539F\\u5206\\u9662\\u5716\\u66F8\\u5340', 286, true)&quot;); 
window.dlocation.add(983,1,&quot;z3llc - z3llc&quot;, &quot;javascript:window.dlocation.selectElement('z3llc - z3llc', 983, true)&quot;); 
window.dlocation.add(647,1,&quot;z6bkf - z6bkf&quot;, &quot;javascript:window.dlocation.selectElement('z6bkf - z6bkf', 647, true)&quot;); 
window.dlocation.add(646,1,&quot;zd1a2 - zd1a2&quot;, &quot;javascript:window.dlocation.selectElement('zd1a2 - zd1a2', 646, true)&quot;); 
window.dlocation.add(663,1,&quot;zd1e - zd1e&quot;, &quot;javascript:window.dlocation.selectElement('zd1e - zd1e', 663, true)&quot;); 
window.dlocation.add(644,1,&quot;zdlf - zdlf&quot;, &quot;javascript:window.dlocation.selectElement('zdlf - zdlf', 644, true)&quot;); 
window.dlocation.add(403,1,&quot;\u4E94\u6A13\u6F2B\u756B\u66F8\u5C08\u5340 - \u4E94\u6A13\u6F2B\u756B\u66F8\u5C08\u5340&quot;, &quot;javascript:window.dlocation.selectElement('\\u4E94\\u6A13\\u6F2B\\u756B\\u66F8\\u5C08\\u5340 - \\u4E94\\u6A13\\u6F2B\\u756B\\u66F8\\u5C08\\u5340', 403, true)&quot;); 
window.dlocation.add(563,1,&quot;\u5BE6\u9AD4\u9928\u85CF - \u5BE6\u9AD4\u9928\u85CF&quot;, &quot;javascript:window.dlocation.selectElement('\\u5BE6\\u9AD4\\u9928\\u85CF - \\u5BE6\\u9AD4\\u9928\\u85CF', 563, true)&quot;); 
window.dlocation.add(1383,1,&quot;\u7D9C\u5408\u66F8\u5EAB - \u7D9C\u5408\u66F8\u5EAB&quot;, &quot;javascript:window.dlocation.selectElement('\\u7D9C\\u5408\\u66F8\\u5EAB - \\u7D9C\\u5408\\u66F8\\u5EAB', 1383, true)&quot;); 
window.dlocation.add(383,1,&quot;\u827E\u8FEA\u8A0A\u5716\u66F8\u9928 - \u827E\u8FEA\u8A0A\u5716\u66F8\u9928&quot;, &quot;javascript:window.dlocation.selectElement('\\u827E\\u8FEA\\u8A0A\\u5716\\u66F8\\u9928 - \\u827E\\u8FEA\\u8A0A\\u5716\\u66F8\\u9928', 383, true)&quot;); 
window.dlocation.add(1384,1,&quot;\u9644\u4E2D\u51FA\u7248\u7269\u5C08\u5340 - \u9644\u4E2D\u51FA\u7248\u7269\u5C08\u5340&quot;, &quot;javascript:window.dlocation.selectElement('\\u9644\\u4E2D\\u51FA\\u7248\\u7269\\u5C08\\u5340 - \\u9644\\u4E2D\\u51FA\\u7248\\u7269\\u5C08\\u5340', 1384, true)&quot;); 
window.dlocation.add(443,1,&quot;\u9F8D\u83EF\u79D1\u6280\u5927\u5B78\u5716\u66F8\u9928 - \u9F8D\u83EF\u79D1\u6280\u5927\u5B78\u5716\u66F8\u9928&quot;, &quot;javascript:window.dlocation.selectElement('\\u9F8D\\u83EF\\u79D1\\u6280\\u5927\\u5B78\\u5716\\u66F8\\u9928 - \\u9F8D\\u83EF\\u79D1\\u6280\\u5927\\u5B78\\u5716\\u66F8\\u9928', 443, true)&quot;); 
window.dlocation.selectElement = function(lname, id, hideTree) { 
document.getElementById('location_0').value = id; 
document.getElementById('elementName').value = lname; 
if(hideTree == true) changeStatus('locationTree'); 
}; 
 document.getElementById('locationArea').innerHTML =  window.dlocation; 
  
  
  


						
				
			
			
				
					
						請輸入要修改的條碼號
						
條碼號

						
							
							

//&lt;![CDATA[

								document.getElementById(&quot;listField&quot;).focus();
							
//]]&gt;


						
						
						
						
					
				
			
		

		
			
				注意:若是修改的資料內容與原始內容相同，則不修改亦不顯示。
			

			
		
		

			
				
					
						
							
								
									
										排序條件:   
條碼號
分類號

										
									
								
								
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
500
 
			
		

		
			

        	

	






            




	jQuery.noConflict();
	var popNewBookSelect={
	          node_this:'',
			  code:'',       
			  name:'',
			  popUp : function(codePtr,namePtr,top,left,obj){//POPUP WINDOW------------------------------------------------------
			          (function($){
						  $(&quot;#popNewBookSelect&quot;).css({'top' : top ,'left' : left});//位置設定
						  popNewBookSelect.node_this  = $(obj);
						  popNewBookSelect.code  	   = codePtr;
						  popNewBookSelect.name  	   = namePtr;
				          //var dropdownSet = $('#popNewBookSelect #L1dropdown');
		                  popNewBookSelect.init();
					  })(jQuery);
			  },
              init : function(){
			          (function($){
						    var contextPath=trim($('#popNewBookSelect #contextPath').text());//取contextPath
			                $('#popNewBookSelect img.clear').attr(&quot;src&quot;,contextPath+&quot;images/clear.gif&quot;);
			                $('#popNewBookSelect img.wait').attr(&quot;src&quot;,contextPath+&quot;images/wait.gif&quot;);
			                //alert($('#popNewBookSelect img.clear').attr(&quot;src&quot;));
			                $(&quot;#popNewBookSelect input.button&quot;).val($('#popNewBookSelect #save').text());
			              	$(&quot;#popNewBookSelect&quot;).show();
					  })(jQuery);
              },
		      cancel : function() {//CANCEL----------------------------------------------------------------------------
					(function($){
							$(&quot;#popNewBookSelect&quot;).hide();
					})(jQuery);
			  },
		      checkBoxClear : function() {//checkBox Clear----------------------------------------------------------------------------
					(function($){
						$(&quot;#JQ_ItemCRS input.check&quot;).each(function(){
								$(this).attr(&quot;checked&quot;,false);
						});
						$(&quot;#rootCheck&quot;).attr(&quot;checked&quot;,false);
					})(jQuery);
			  },
		      confirm : function() {//run--------------------------------------------------------------------------
					(function($){
						$(&quot;#popNewBookSelect&quot;).hide();
						var contextPath=trim($('#popNewBookSelect #contextPath').text());//取contextPath
						var year =$('#year  :selected').text();
						var month=$('#month :selected').text();
					    var i=0;
					    var flag=&quot;n&quot;;
					    var selectItems=&quot;&quot;;
						$(&quot;#JQ_ItemCRS input.check&quot;).each(function(){
								//alert(i+&quot; - &quot;+$(this).attr(&quot;checked&quot;));
								var select=$(this).attr(&quot;checked&quot;);
								if(select==true){
									//alert(i+&quot; - &quot;+$(this).parent().parent().find(&quot;td[className=itemId]&quot;).text());
									var itemId=trim($(this).parent().parent().find(&quot;td[className=itemId]&quot;).text());
									if(flag==&quot;y&quot;) selectItems=selectItems+&quot;,&quot;;
									selectItems=selectItems+itemId;
									flag=&quot;y&quot;;
								}
								i++;
						});
						//alert(&quot;REQ=&quot;+selectItems);
						//alert(&quot;year=&quot;+year);
						//alert(&quot;month=&quot;+month);
						popNewBookSelect.checkBoxClear();
				        $.ajax({ 
							    type: &quot;POST&quot;, 
							    url: contextPath+&quot;/kidMain&quot;, 
							    dataType: &quot;text&quot;, 
							    data: {method:&quot;setNewBooks&quot;,selectItems:selectItems,year:year,month:month}, 
							    success: function(html) {
										//alert(&quot;RES=&quot;+html);
										var success=$('#popNewBookSelect #success').text();
										alert(success);
							    }, 
							    error: function (XMLHttpRequest, textStatus, errorThrown) { 
							    	alert(XMLHttpRequest.responseText);
							    } 
						});
					})(jQuery);
			  }
	}	  


&lt;!--
	#popNewBookSelect {
		position: absolute;
		left: 250px;
		top: 200px;
		z-index: 1;
	}
	#popNewBookSelect .pop_foreground {
	z-index:3;
	left: 21px;
	background-color: #FFFFFF;
	position: absolute;
	width: 342px;
	border-top-width: 1px;
	border-right-width: 1px;
	border-bottom-width: 1px;
	border-left-width: 1px;
	border-top-style: solid;
	border-right-style: solid;
	border-bottom-style: solid;
	border-left-style: solid;
	border-top-color: #84A9D8;
	border-right-color: #84A9D8;
	border-bottom-color: #84A9D8;
	border-left-color: #84A9D8;
	}
	#popNewBookSelect .pop_foreground .pop_cancel {
		position: absolute;
		top: 6px;
		right: 10px;
	}
	#popNewBookSelect .pop_foreground .pop_cancel a {
		color: #FF0000;
		text-decoration: none;
	}
	#popNewBookSelect .pop_foreground  .pop_header {
	color: #FFFFFF;
	background-color: #84A9D8;
	margin-top: 0px;
	margin-bottom: 0px;
	}
	#popNewBookSelect .pop_foreground  .pop_header p {
		margin-top: 0px;
		margin-bottom: 0px;
		font-size: 12pt;
		line-height: 2em;
		font-weight: bold;
	}
	#popNewBookSelect .pop_foreground .pop_content {
		width: 100%;
		margin-top: 0px;
		margin-bottom: 0px;
		border: 1px solid #666666;
		background: #FFFFFF;
	}
	#popNewBookSelect .pop_foreground .pop_content a {
		text-decoration: none;
		font-size: 12pt;
		line-height: 1em;
		color: #000000;
		display: block;
	}
	#popNewBookSelect .pop_foreground .pop_content a:hover {
		text-decoration: none;
		font-size: 12pt;
		line-height: 1em;
		color: #FFFFFF;
		background: #a69f00;
	}
	#popNewBookSelect .pop_foreground  .pop_content p {
		margin-top: 0px;
		margin-bottom: 0px;
		font-size: 12pt;
		line-height: 1em;
	}
	#popNewBookSelect .pop_foreground .pop_footer {
		background: #FFA100;
		text-align: center;
		font-size: 12pt;
		line-height: 1em;
		color: #993300;
	}

	#popNewBookSelect #detailFormContainer {
	width: 322px;
	padding-top: 2px;
	padding-right: 10px;
	padding-bottom: 12px;
	padding-left: 10px;
	}
	#popNewBookSelect #detailFormContainer br {
			clear: both;
	}
	#popNewBookSelect #detailFormContainer #cascadingDropdowns div {
		  float: left;
		  margin-right: 10px;
	}
	#popNewBookSelect #detailFormContainer label {
			float: left;
			margin-right: 10px;
			color: #FFFFFF;
			font: 24px &quot;標楷體&quot;;
	}
#popNewBookSelect #confirm {
	text-align: right;
	margin-left: 225px;
	margin-top: 20px;
}
#popNewBookSelect #confirm .button {
	background-color: #84A9D8;
	margin: 0px;
	border: 1px outset #CCC;
}
-->



		/inspireapp/
		
		
	    
	    	
	        
	          	
	        
	        
      			
                    
                    
                          
                          	
							      
									  
									        2012
									        2011
									        2010
									        2009
									        2008
									        2007
									        2006
									        2005
									        2004
									        2003
									        2002
									        2001
									        2000
								      
							      
							           
									  
									          01
									          02
									          03
									          04
									          05
									          06
									          07
									          08
									          09
									          10
									          11
									          12
								      
							      
							  
	                          
 		    						
							  
                          
                    
                    
                
            
	    

	
	

        	




	jQuery.noConflict();
	var popGoodBookSelect={
	          node_this:'',
			  code:'',       
			  name:'',
			  popUp : function(codePtr,namePtr,top,left,obj){//POPUP WINDOW------------------------------------------------------
			          (function($){
						  $(&quot;#popGoodBookSelect&quot;).css({'top' : top ,'left' : left});//位置設定
						  popGoodBookSelect.node_this  = $(obj);
						  popGoodBookSelect.code  	   = codePtr;
						  popGoodBookSelect.name  	   = namePtr;
		                  popGoodBookSelect.init();
					  })(jQuery);
			  },
              init : function(){
			          (function($){
						    var contextPath=trim($('#popGoodBookSelect #contextPath').text());//取contextPath
			                $('#popGoodBookSelect img.clear').attr(&quot;src&quot;,contextPath+&quot;images/clear.gif&quot;);
			                $('#popGoodBookSelect img.wait').attr(&quot;src&quot;,contextPath+&quot;images/wait.gif&quot;);
					        $.ajax({ 
								    type: &quot;POST&quot;, 
								    url: contextPath+&quot;/kidMain&quot;, 
								    dataType: &quot;json&quot;, 
								    data: {method:&quot;getGoodBookItemCode&quot;,language:&quot;4&quot;,ignore:&quot;00&quot;}, 
								    success: function(data) {
										  //alert(&quot;RES=&quot;+data);
						              	  $(&quot;#popGoodBookSelect #L1DropDown&quot;).loadSelect(data);
								    }, 
								    error: function (XMLHttpRequest, textStatus, errorThrown) { 
								    	alert(XMLHttpRequest.responseText);
								    } 
							});
			                $(&quot;#popGoodBookSelect input.button&quot;).val($('#popGoodBookSelect #save').text());
			              	$(&quot;#popGoodBookSelect&quot;).show();
					  })(jQuery);
              },
		      cancel : function() {//CANCEL----------------------------------------------------------------------------
					(function($){
							$(&quot;#popGoodBookSelect&quot;).hide();
					})(jQuery);
			  },
		      checkBoxClear : function() {//checkBox Clear----------------------------------------------------------------------------
					(function($){
						$(&quot;#JQ_ItemCRS input.check&quot;).each(function(){
								$(this).attr(&quot;checked&quot;,false);
						});
						$(&quot;#rootCheck&quot;).attr(&quot;checked&quot;,false);
					})(jQuery);
			  },
		      confirm : function() {//run--------------------------------------------------------------------------
					(function($){
						$(&quot;#popGoodBookSelect&quot;).hide();
						var contextPath=trim($('#popGoodBookSelect #contextPath').text());//取contextPath
						//var id =$('#popGoodBookSelect #topicId  :selected').val();
						var id=$('#popGoodBookSelect #L1DropDown').val();
					    var i=0;
					    var flag=&quot;n&quot;;
					    var selectItems=&quot;&quot;;
						$(&quot;#JQ_ItemCRS input.check&quot;).each(function(){
								//alert(i+&quot; - &quot;+$(this).attr(&quot;checked&quot;));
								var select=$(this).attr(&quot;checked&quot;);
								if(select==true){
									//alert(i+&quot; - &quot;+$(this).parent().parent().find(&quot;td[className=itemId]&quot;).text());
									var itemId=trim($(this).parent().parent().find(&quot;td[className=itemId]&quot;).text());
									if(flag==&quot;y&quot;) selectItems=selectItems+&quot;,&quot;;
									selectItems=selectItems+itemId;
									flag=&quot;y&quot;;
								}
								i++;
						});
						//alert(&quot;good book REQ=&quot;+selectItems);
						//alert(&quot;good book topc-id=&quot;+id);
						popGoodBookSelect.checkBoxClear();
				        $.ajax({ 
							    type: &quot;POST&quot;, 
							    url: contextPath+&quot;/kidMain&quot;, 
							    dataType: &quot;text&quot;, 
							    data: {method:&quot;setGoodBooks&quot;,selectItems:selectItems,id:id}, 
							    success: function(html) {
										//alert(&quot;RES=&quot;+html);
										var success=$('#popGoodBookSelect #success').text();
										alert(success);
							    }, 
							    error: function (XMLHttpRequest, textStatus, errorThrown) { 
							    	alert(XMLHttpRequest.responseText);
							    } 
						});
					})(jQuery);
			  }
	}	  


&lt;!--
	#popGoodBookSelect {
		position: absolute;
		left: 250px;
		top: 200px;
		z-index: 1;
	}
	#popGoodBookSelect .pop_foreground {
	z-index:3;
	left: 21px;
	background-color: #FFFFFF;
	position: absolute;
	width: 342px;
	border-top-width: 1px;
	border-right-width: 1px;
	border-bottom-width: 1px;
	border-left-width: 1px;
	border-top-style: solid;
	border-right-style: solid;
	border-bottom-style: solid;
	border-left-style: solid;
	border-top-color: #84A9D8;
	border-right-color: #84A9D8;
	border-bottom-color: #84A9D8;
	border-left-color: #84A9D8;
	}
	#popGoodBookSelect .pop_foreground .pop_cancel {
		position: absolute;
		top: 6px;
		right: 10px;
	}
	#popGoodBookSelect .pop_foreground .pop_cancel a {
		color: #FF0000;
		text-decoration: none;
	}
	#popGoodBookSelect .pop_foreground  .pop_header {
	color: #FFFFFF;
	background-color: #84A9D8;
	margin-top: 0px;
	margin-bottom: 0px;
	}
	#popGoodBookSelect .pop_foreground  .pop_header p {
		margin-top: 0px;
		margin-bottom: 0px;
		font-size: 12pt;
		line-height: 2em;
		font-weight: bold;
	}
	#popGoodBookSelect .pop_foreground .pop_content {
		width: 100%;
		margin-top: 0px;
		margin-bottom: 0px;
		border: 1px solid #666666;
		background: #FFFFFF;
	}
	#popGoodBookSelect .pop_foreground .pop_content a {
		text-decoration: none;
		font-size: 12pt;
		line-height: 1em;
		color: #000000;
		display: block;
	}
	#popGoodBookSelect .pop_foreground .pop_content a:hover {
		text-decoration: none;
		font-size: 12pt;
		line-height: 1em;
		color: #FFFFFF;
		background: #a69f00;
	}
	#popGoodBookSelect .pop_foreground  .pop_content p {
		margin-top: 0px;
		margin-bottom: 0px;
		font-size: 12pt;
		line-height: 1em;
	}
	#popGoodBookSelect .pop_foreground .pop_footer {
		background: #FFA100;
		text-align: center;
		font-size: 12pt;
		line-height: 1em;
		color: #993300;
	}

	#popGoodBookSelect #detailFormContainer {
	width: 322px;
	padding-top: 2px;
	padding-right: 10px;
	padding-bottom: 12px;
	padding-left: 10px;
	}
	#popGoodBookSelect #detailFormContainer br {
			clear: both;
	}
	#popGoodBookSelect #detailFormContainer #cascadingDropdowns div {
		  float: left;
		  margin-right: 10px;
	}
	#popGoodBookSelect #detailFormContainer label {
			float: left;
			margin-right: 10px;
			color: #FFFFFF;
			font: 24px &quot;標楷體&quot;;
	}
#popGoodBookSelect #confirm {
	text-align: right;
	margin-left: 225px;
	margin-top: 20px;
}
#popGoodBookSelect #confirm .button {
	background-color: #84A9D8;
	margin: 0px;
	border: 1px outset #CCC;
}
-->



		/inspireapp/
		
		
	    
	    	
	        
	          	Good Book Setup
	        
	        
      			
                    
                    
                          
                          	  
							      
	                              
							      
							  
	                          
 		    						
							  
                          
                    
                    
                
            
	    

	
	

        	




	jQuery.noConflict();
	var popTopicBookSelect={
	          node_this:'',
			  code:'',       
			  name:'',
			  popUp : function(codePtr,namePtr,top,left,obj){//POPUP WINDOW------------------------------------------------------
			          (function($){
						  $(&quot;#popTopicBookSelect&quot;).css({'top' : top ,'left' : left});//位置設定
						  popTopicBookSelect.node_this  = $(obj);
						  popTopicBookSelect.code  	   = codePtr;
						  popTopicBookSelect.name  	   = namePtr;
		                  popTopicBookSelect.init();
					  })(jQuery);
			  },
              init : function(){
			          (function($){
						    var contextPath=trim($('#popTopicBookSelect #contextPath').text());//取contextPath
			                $('#popTopicBookSelect img.clear').attr(&quot;src&quot;,contextPath+&quot;images/clear.gif&quot;);
			                $('#popTopicBookSelect img.wait').attr(&quot;src&quot;,contextPath+&quot;images/wait.gif&quot;);
					        $.ajax({ 
								    type: &quot;POST&quot;, 
								    url: contextPath+&quot;/kidMain&quot;, 
								    dataType: &quot;json&quot;, 
								    data: {method:&quot;getTopicBookMainItemCode&quot;,language:&quot;4&quot;}, 
								    success: function(data) {
										  //alert(&quot;RES=&quot;+data);
						              	  $(&quot;#popTopicBookSelect #L1DropDown&quot;).loadSelect(data);
						              	  popTopicBookSelect.L2_Dropdown();
								    }, 
								    error: function (XMLHttpRequest, textStatus, errorThrown) { 
								    	alert(XMLHttpRequest.responseText);
								    } 
							});
							//bind
					        $('#popTopicBookSelect #L1dropdown').change(function(){
						            popTopicBookSelect.L2_Dropdown();
					        });
			                $(&quot;#popTopicBookSelect input.button&quot;).val($('#popTopicBookSelect #save').text());
			              	$(&quot;#popTopicBookSelect&quot;).show();
					  })(jQuery);
              },
		      L2_Dropdown : function() {//LEVEL 2----------------------------------------------------------------------------------
					(function($){
				        var L1_value    = $('#popTopicBookSelect #L1dropdown').val();
				        var dropdownSet = $('#popTopicBookSelect #L2dropdown');
				        if (L1_value.length == 0) {
					          dropdownSet.attr(&quot;disabled&quot;,true);
					          dropdownSet.emptySelect();
				        }else {
					          dropdownSet.attr(&quot;disabled&quot;,false);
						      var contextPath=trim($('#popTopicBookSelect #contextPath').text());//取contextPath
					          $.ajax({ 
									type: &quot;POST&quot;, 
									url: contextPath+&quot;/kidMain&quot;, 
									dataType: &quot;json&quot;, 
									data: {method:&quot;getTopicBookSubItemCode&quot;,main_id:trim(L1_value),language:&quot;4&quot;}, 
								    success: function(data) {
						            	dropdownSet.loadSelect(data);
								    }, 
								    error: function (XMLHttpRequest, textStatus, errorThrown) { 
								    	//alert(XMLHttpRequest.responseText);
								    	alert(&quot;no data&quot;);
								    } 
							  });
				        }
					})(jQuery);
		      },
		      cancel : function() {//CANCEL----------------------------------------------------------------------------
					(function($){
							$(&quot;#popTopicBookSelect&quot;).hide();
					})(jQuery);
			  },
		      checkBoxClear : function() {//checkBox Clear----------------------------------------------------------------------------
					(function($){
						$(&quot;#JQ_ItemCRS input.check&quot;).each(function(){
								$(this).attr(&quot;checked&quot;,false);
						});
						$(&quot;#rootCheck&quot;).attr(&quot;checked&quot;,false);
					})(jQuery);
			  },
		      confirm : function() {//run--------------------------------------------------------------------------
					(function($){
							$(&quot;#popTopicBookSelect&quot;).hide();
							var contextPath=trim($('#popTopicBookSelect #contextPath').text());//取contextPath
						    var i=0;
						    var flag=&quot;n&quot;;
						    var selectItems=&quot;&quot;;
							$(&quot;#JQ_ItemCRS input.check&quot;).each(function(){
									//alert(i+&quot; - &quot;+$(this).attr(&quot;checked&quot;));
									var select=$(this).attr(&quot;checked&quot;);
									if(select==true){
										//alert(i+&quot; - &quot;+$(this).parent().parent().find(&quot;td[className=itemId]&quot;).text());
										var itemId=trim($(this).parent().parent().find(&quot;td[className=itemId]&quot;).text());
										if(flag==&quot;y&quot;) selectItems=selectItems+&quot;,&quot;;
										selectItems=selectItems+itemId;
										flag=&quot;y&quot;;
									}
									i++;
							});
							//alert(&quot;good book REQ=&quot;+selectItems);
					        var mainId =$(&quot;#popTopicBookSelect #L1dropdown&quot;).find(':selected').val();
					        var subId  =$(&quot;#popTopicBookSelect #L2dropdown&quot;).find(':selected').val();
					        //alert(mainId+&quot;/&quot;+subId);
							popTopicBookSelect.checkBoxClear();
					        $.ajax({ 
								    type: &quot;POST&quot;, 
								    url: contextPath+&quot;/kidMain&quot;, 
								    dataType: &quot;text&quot;, 
								    data: {method:&quot;setTopicBooks&quot;,selectItems:selectItems,mainId:mainId,subId:subId}, 
								    success: function(html) {
											//alert(&quot;RES=&quot;+html);
											var success=$('#popTopicBookSelect #success').text();
											alert(success);
								    }, 
								    error: function (XMLHttpRequest, textStatus, errorThrown) { 
								    	alert(XMLHttpRequest.responseText);
								    } 
							});
					})(jQuery);
			  }
	}	  


&lt;!--
	#popTopicBookSelect {
		position: absolute;
		left: 250px;
		top: 200px;
		z-index: 1;
	}
	#popTopicBookSelect .pop_foreground {
	z-index:3;
	left: 21px;
	background-color: #FFFFFF;
	position: absolute;
	width: 342px;
	border-top-width: 1px;
	border-right-width: 1px;
	border-bottom-width: 1px;
	border-left-width: 1px;
	border-top-style: solid;
	border-right-style: solid;
	border-bottom-style: solid;
	border-left-style: solid;
	border-top-color: #84A9D8;
	border-right-color: #84A9D8;
	border-bottom-color: #84A9D8;
	border-left-color: #84A9D8;
	}
	#popTopicBookSelect .pop_foreground .pop_cancel {
		position: absolute;
		top: 6px;
		right: 10px;
	}
	#popTopicBookSelect .pop_foreground .pop_cancel a {
		color: #FF0000;
		text-decoration: none;
	}
	#popTopicBookSelect .pop_foreground  .pop_header {
	color: #FFFFFF;
	background-color: #84A9D8;
	margin-top: 0px;
	margin-bottom: 0px;
	}
	#popTopicBookSelect .pop_foreground  .pop_header p {
		margin-top: 0px;
		margin-bottom: 0px;
		font-size: 12pt;
		line-height: 2em;
		font-weight: bold;
	}
	#popTopicBookSelect .pop_foreground .pop_content {
		width: 100%;
		margin-top: 0px;
		margin-bottom: 0px;
		border: 1px solid #666666;
		background: #FFFFFF;
	}
	#popTopicBookSelect .pop_foreground .pop_content a {
		text-decoration: none;
		font-size: 12pt;
		line-height: 1em;
		color: #000000;
		display: block;
	}
	#popTopicBookSelect .pop_foreground .pop_content a:hover {
		text-decoration: none;
		font-size: 12pt;
		line-height: 1em;
		color: #FFFFFF;
		background: #a69f00;
	}
	#popTopicBookSelect .pop_foreground  .pop_content p {
		margin-top: 0px;
		margin-bottom: 0px;
		font-size: 12pt;
		line-height: 1em;
	}
	#popTopicBookSelect .pop_foreground .pop_footer {
		background: #FFA100;
		text-align: center;
		font-size: 12pt;
		line-height: 1em;
		color: #993300;
	}

	#popTopicBookSelect #detailFormContainer {
	width: 322px;
	padding-top: 2px;
	padding-right: 10px;
	padding-bottom: 12px;
	padding-left: 10px;
	}
	#popTopicBookSelect #detailFormContainer br {
			clear: both;
	}
	#popTopicBookSelect #detailFormContainer #cascadingDropdowns div {
		  float: left;
		  margin-right: 10px;
	}
	#popTopicBookSelect #detailFormContainer label {
			float: left;
			margin-right: 10px;
			color: #FFFFFF;
			font: 24px &quot;標楷體&quot;;
	}
#popTopicBookSelect #confirm {
	text-align: right;
	margin-left: 225px;
	margin-top: 20px;
}
#popTopicBookSelect #confirm .button {
	background-color: #84A9D8;
	margin: 0px;
	border: 1px outset #CCC;
}
-->



		/inspireapp/
		
		
	    
	    	
	        
	          	Good Book Setup
	        
	        
      			
                    
                    
                          
	                          
	                            primary
	                            
	                          
	                          
	                            secondary
	                            
	                          
	                          
 		    						
							  
                          
                    
                    
                
            
	    

	
	

	 

			


		

		 

	

	

 
  
    Go To Page
  
  
 
 
  
 


 
		
	 






	
	var tm_timerID = window.setInterval(&quot;refreshTaskManager()&quot;, 10000);
	var init = 0;
	function refreshTaskManager() {
		var status = 'S1';
		var execute = 0;
		if (document.getElementById('tmArea') != null) {
			execute = 1;
			//status = document.getElementById('taskStatus').value;
		}
		if (execute == 1) {
			if ((status == 'S1') || (init == 0)) {
				document.getElementById('taskManagerRefreshLink').onclick();
			} else {
				clearInterval(tm_timerID);
			}
			init++;
		}
	}
	


 
  
  
     
  
 
 
  
 



	
	


 
  
    更改館藏狀態
  
  
 
 
  
 




		
	var tm_timerID = window.setInterval(&quot;refreshTaskManager()&quot;, 10000);
	var init = 0;
	function refreshTaskManager() {
		var status = 'S1';
		var execute = 0;
		if (document.getElementById('tmArea') != null) {
			execute = 1;
			//status = document.getElementById('taskStatus').value;
		}
		if (execute == 1) {
			if ((status == 'S1') || (init == 0)) {
				document.getElementById('taskManagerRefreshLink').onclick();
			} else {
				clearInterval(tm_timerID);
			}
			init++;
		}
	}
	


 
  
  
     
  
 
 
  
 




		
	

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
var field1 = new Ajax.Autocompleter(&quot;field1&quot;, &quot;field1choices&quot;, &quot;/inspireapp/internaltranzit/ManageInBatch,searchComponent.field1.sdirect?sp=Sfield1&amp;sp=S2&amp;sp=Sstarts+with&amp;sp=1&amp;updateParts=field1&quot;, {&quot;method&quot;:&quot;get&quot;,&quot;frequency&quot;:0.2,&quot;minChars&quot;:1,&quot;onFailure&quot;:tapestry.error,&quot;encoding&quot;:&quot;UTF-8&quot;});
var field2 = new Ajax.Autocompleter(&quot;field2&quot;, &quot;field2choices&quot;, &quot;/inspireapp/internaltranzit/ManageInBatch,searchComponent.field2.sdirect?sp=Sfield2&amp;sp=S30&amp;sp=Sstarts+with&amp;sp=2&amp;updateParts=field2&quot;, {&quot;method&quot;:&quot;get&quot;,&quot;frequency&quot;:0.2,&quot;minChars&quot;:1,&quot;onFailure&quot;:tapestry.error,&quot;encoding&quot;:&quot;UTF-8&quot;});
var field3 = new Ajax.Autocompleter(&quot;field3&quot;, &quot;field3choices&quot;, &quot;/inspireapp/internaltranzit/ManageInBatch,searchComponent.field3.sdirect?sp=Sfield3&amp;sp=S5&amp;sp=Sstarts+with&amp;sp=3&amp;updateParts=field3&quot;, {&quot;method&quot;:&quot;get&quot;,&quot;frequency&quot;:0.2,&quot;minChars&quot;:1,&quot;onFailure&quot;:tapestry.error,&quot;encoding&quot;:&quot;UTF-8&quot;});
var field4 = new Ajax.Autocompleter(&quot;field4&quot;, &quot;field4choices&quot;, &quot;/inspireapp/internaltranzit/ManageInBatch,searchComponent.field4.sdirect?sp=Sfield4&amp;sp=S7&amp;sp=Sstarts+with&amp;sp=4&amp;updateParts=field4&quot;, {&quot;method&quot;:&quot;get&quot;,&quot;frequency&quot;:0.2,&quot;minChars&quot;:1,&quot;onFailure&quot;:tapestry.error,&quot;encoding&quot;:&quot;UTF-8&quot;});
tapestry.cleanConnect(&quot;formSubmitSearch&quot;, &quot;onclick&quot;, &quot;event1167465096&quot;);
        tapestry.event1167465096=function(e){
            tapestry.event.stopEvent(e);
            
                
                    tapestry.form.submit(&quot;searchForm&quot;, &quot;formSubmitSearch&quot;, {&quot;async&quot;:true,&quot;json&quot;:false,&quot;url&quot;:&quot;/inspireapp/internaltranzit/ManageInBatch,searchComponent.searchForm.sdirect?updateParts=results&amp;updateParts=nrResults&amp;updateParts=autoRefreshZone&amp;updateParts=zSources&amp;updateParts=localSources&amp;updateParts=showOrderBy&amp;updateParts=noticeMessage&quot;});
                
                
            
            
        };
        tapestry.connect(&quot;formSubmitSearch&quot;, &quot;onclick&quot;, &quot;event1167465096&quot;);
tapestry.cleanConnect(&quot;browse&quot;, &quot;onclick&quot;, &quot;event1984482014&quot;);
        tapestry.event1984482014=function(e){
            tapestry.event.stopEvent(e);
            
                
                    tapestry.form.submit(&quot;searchForm&quot;, &quot;browse&quot;, {&quot;async&quot;:true,&quot;json&quot;:false,&quot;url&quot;:&quot;/inspireapp/internaltranzit/ManageInBatch,searchComponent.searchForm.sdirect?updateParts=results&amp;updateParts=nrResults&amp;updateParts=autoRefreshZone&amp;updateParts=inputField5&quot;});
                
                
            
            
        };
        tapestry.connect(&quot;browse&quot;, &quot;onclick&quot;, &quot;event1984482014&quot;);
tapestry.cleanConnect(&quot;Submit&quot;, &quot;onclick&quot;, &quot;event178957379&quot;);
        tapestry.event178957379=function(e){
            tapestry.event.stopEvent(e);
            
                
                    tapestry.form.submit(&quot;searchForm&quot;, &quot;Submit&quot;, {&quot;async&quot;:true,&quot;json&quot;:false,&quot;url&quot;:&quot;/inspireapp/internaltranzit/ManageInBatch,searchComponent.searchForm.sdirect?updateParts=results&amp;updateParts=nrResults&amp;updateParts=autoRefreshZone&amp;updateParts=inputField5&quot;});
                
                
            
            
        };
        tapestry.connect(&quot;Submit&quot;, &quot;onclick&quot;, &quot;event178957379&quot;);

tapestry.cleanConnect(&quot;sCriteria1&quot;, &quot;onchange&quot;, &quot;formEvent1172116752&quot;);
                tapestry.formEvent1172116752=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;internaltranzit/ManageInBatch/searchComponent.sCriteria1&quot;, bcomponentid:&quot;sCriteria1&quot;};
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
                tapestry.connect(&quot;sCriteria1&quot;, &quot;onchange&quot;, &quot;formEvent1172116752&quot;);
tapestry.cleanConnect(&quot;comparator1&quot;, &quot;onchange&quot;, &quot;formEvent308899038&quot;);
                tapestry.formEvent308899038=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;internaltranzit/ManageInBatch/searchComponent.comparator1&quot;, bcomponentid:&quot;comparator1&quot;};
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
                tapestry.connect(&quot;comparator1&quot;, &quot;onchange&quot;, &quot;formEvent308899038&quot;);
tapestry.cleanConnect(&quot;sCriteria2&quot;, &quot;onchange&quot;, &quot;formEvent1335983496&quot;);
                tapestry.formEvent1335983496=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;internaltranzit/ManageInBatch/searchComponent.sCriteria2&quot;, bcomponentid:&quot;sCriteria2&quot;};
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
                tapestry.connect(&quot;sCriteria2&quot;, &quot;onchange&quot;, &quot;formEvent1335983496&quot;);
tapestry.cleanConnect(&quot;comparator2&quot;, &quot;onchange&quot;, &quot;formEvent1479350283&quot;);
                tapestry.formEvent1479350283=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;internaltranzit/ManageInBatch/searchComponent.comparator2&quot;, bcomponentid:&quot;comparator2&quot;};
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
                tapestry.connect(&quot;comparator2&quot;, &quot;onchange&quot;, &quot;formEvent1479350283&quot;);
tapestry.cleanConnect(&quot;sCriteria3&quot;, &quot;onchange&quot;, &quot;formEvent886327579&quot;);
                tapestry.formEvent886327579=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;internaltranzit/ManageInBatch/searchComponent.sCriteria3&quot;, bcomponentid:&quot;sCriteria3&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;sCriteria3&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;sCriteria3&quot;, &quot;onchange&quot;, &quot;formEvent886327579&quot;);
tapestry.cleanConnect(&quot;comparator3&quot;, &quot;onchange&quot;, &quot;formEvent1930191548&quot;);
                tapestry.formEvent1930191548=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;internaltranzit/ManageInBatch/searchComponent.comparator3&quot;, bcomponentid:&quot;comparator3&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;comparator3&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;comparator3&quot;, &quot;onchange&quot;, &quot;formEvent1930191548&quot;);
tapestry.cleanConnect(&quot;sCriteria4&quot;, &quot;onchange&quot;, &quot;formEvent588250658&quot;);
                tapestry.formEvent588250658=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;internaltranzit/ManageInBatch/searchComponent.sCriteria4&quot;, bcomponentid:&quot;sCriteria4&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;sCriteria4&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;sCriteria4&quot;, &quot;onchange&quot;, &quot;formEvent588250658&quot;);
tapestry.cleanConnect(&quot;comparator4&quot;, &quot;onchange&quot;, &quot;formEvent851833462&quot;);
                tapestry.formEvent851833462=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;internaltranzit/ManageInBatch/searchComponent.comparator4&quot;, bcomponentid:&quot;comparator4&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;comparator4&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;comparator4&quot;, &quot;onchange&quot;, &quot;formEvent851833462&quot;);
tapestry.cleanConnect(&quot;browseCriteria&quot;, &quot;onchange&quot;, &quot;formEvent1302783030&quot;);
                tapestry.formEvent1302783030=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;internaltranzit/ManageInBatch/searchComponent.browseCriteria&quot;, bcomponentid:&quot;browseCriteria&quot;};
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
                tapestry.connect(&quot;browseCriteria&quot;, &quot;onchange&quot;, &quot;formEvent1302783030&quot;);
closeDialogComponent('TinreadDialog');

closeDialogComponent('GlobalChangeStatus');
try {
  attachFocus('field1');
 }
 catch(e) {}
closeDialogComponent('IGCD');
closeDialogComponent('GlobalChangeStatus_1');
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



            
                
                    確定
                    取消
                
                
                    
                        
                            CMUL - 神資圖書館
                            
                                
                            
                        
                    
                    
                    
                        
                            2 - 2
                            
                                
                            
                        
                    
                    
                
                    
                        
                            123 - 123
                            
                                
                            
                        
                    
                    
                
                    
                        
                            20230417 - 20230417
                            
                                
                            
                        
                    
                    
                
                    
                        
                            20230418 - 20230418
                            
                                
                            
                        
                    
                    
                
                    
                        
                            AH - 安南醫院
                            
                                
                            
                        
                    
                    
                    
                        
                            AHGL - 安南圖書區
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            av - av
                            
                                
                            
                        
                    
                    
                
                    
                        
                            B007 - B007
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BCSB4 - BCSB4
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BX - 取書櫃1
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BY - 取書櫃2
                            
                                
                            
                        
                    
                    
                
                    
                        
                            CB - 北港分館
                            
                                
                            
                        
                    
                    
                    
                        
                            BAVN - 北港分館視聽區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BCIR - 北港分館流通櫃檯
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BCRA - 北港分館指參(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BPAV - 北港分館視聽區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BPCL - 北港分館書庫
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            cbook - cbook
                            
                                
                            
                        
                    
                    
                
                    
                        
                            circd - circd
                            
                                
                            
                        
                    
                    
                
                    
                        
                            clp - clp
                            
                                
                            
                        
                    
                    
                
                    
                        
                            CM - 北港附設醫院
                            
                                
                            
                        
                    
                    
                    
                        
                            BMHL - 北港附設醫院圖書室
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            CMUL - 中國醫藥大學圖書館 - CMUL - 中國醫藥大學圖書館
                            
                                
                            
                        
                    
                    
                
                    
                        
                            CU - 台中總館
                            
                                
                            
                        
                    
                    
                    
                        
                            CUAV - 台中總館視聽區(獨立專區)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MAVN - 台中總館視聽區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MAVR - 台中總館視聽區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCAT - 台中總館技服組
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCBS - 台中總館密閉書庫
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCIR - 台中總館流通櫃檯
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCRA - 台中總館教師指定參考書(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCSS - 台中總館B3裝訂期刊區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MDIA - 台中總館博碩士論文區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MEAS - 台中總館探索史懷哲之路專書區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MEXM - 台中總館國考書區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MFGA - 台中總館本校教職優良教材區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MFPA - 台中總館本校教師升等資料區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MFSA - 台中總館本校教職論著
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MHME - 台中總館人文專書區-醫學倫理(限館內閱覽)台中總館人文專書區-醫學倫理(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MHMH - 台中總館人文專書區-醫療史(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MHML - 台中總館人文專書區-醫學法律(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MHQC - 台中總館人文專書區-醫療品質(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MNBR - 台中總館新書展示區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MPAA - 台中總館績效暨獲獎區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MPBL - 台中總館PBL專書區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MREF - 台中總館參考室
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MRSS - 台中總館閱覽組
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MS-C - 台中總館期刊複本櫃
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSER - 台中總館期刊區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSPA - 台中總館研究計劃專書
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSPB - 中醫醫史文獻室(限所內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSPC - 台中總館特藏室
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSTK - 台中總館書庫
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MYBK - 台中總館參考壁櫃
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 7 - new item 7
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ONLN - 台中總館線上資料
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            e-resources - 電子資源
                            
                                
                            
                        
                    
                    
                
                    
                        
                            EB-P - EB-P
                            
                                
                            
                        
                    
                    
                
                    
                        
                            elect - elect
                            
                                
                            
                        
                    
                    
                
                    
                        
                            H-EQ - H-EQ
                            
                                
                            
                        
                    
                    
                
                    
                        
                            H-MR - H-MR
                            
                                
                            
                        
                    
                    
                
                    
                        
                            L - L
                            
                                
                            
                        
                    
                    
                
                    
                        
                            L40 - L40
                            
                                
                            
                        
                    
                    
                
                    
                        
                            LB 圖書總館 - LB 圖書總館
                            
                                
                            
                        
                    
                    
                
                    
                        
                            LB-S - LB-S
                            
                                
                            
                        
                    
                    
                
                    
                        
                            LE - 語文教學中心
                            
                                
                            
                        
                    
                    
                    
                        
                            LEGL - 語文教學中心圖書室
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            LIB - LIB
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 1 - new item 1
                            
                                
                            
                        
                    
                    
                    
                        
                            new item 3 - new item 3
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            new item 10 - new item 10
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 12 - new item 12
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 13 - new item 13
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 14 - new item 14
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 16 - new item 16
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 2 - new item 2
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 20 - new item 20
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 4 - new item 4
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 5 - new item 5
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 6 - 英才校區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 8 - new item 8
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 9 - new item 9
                            
                                
                            
                        
                    
                    
                
                    
                        
                            NPTU - NPTU
                            
                                
                            
                        
                    
                    
                
                    
                        
                            OUK - OUK
                            
                                
                            
                        
                    
                    
                
                    
                        
                            PT - 培德醫院
                            
                                
                            
                        
                    
                    
                    
                        
                            new item 11 - new item 11
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 17 - new item 17
                            
                                
                            
                        
                    
                    
                
                    
                        
                            PTGL - 培德醫院圖書區
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            ptext - ptext
                            
                                
                            
                        
                    
                    
                
                    
                        
                            SB3 - SB3
                            
                                
                            
                        
                    
                    
                
                    
                        
                            T-P - T-P
                            
                                
                            
                        
                    
                    
                
                    
                        
                            TBBK - TBBK
                            
                                
                            
                        
                    
                    
                
                    
                        
                            TH - 台北分院
                            
                                
                            
                        
                    
                    
                    
                        
                            THGL - 台北分院圖書區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            THPA - 台北分院期刊區
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            W-P - W-P
                            
                                
                            
                        
                    
                    
                
                    
                        
                            YH - 豐原分院
                            
                                
                            
                        
                    
                    
                    
                        
                            new item 18 - new item 18
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 19 - new item 19
                            
                                
                            
                        
                    
                    
                
                    
                        
                            YHGL - 豐原分院圖書區
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            z3llc - z3llc
                            
                                
                            
                        
                    
                    
                
                    
                        
                            z6bkf - z6bkf
                            
                                
                            
                        
                    
                    
                
                    
                        
                            zd1a2 - zd1a2
                            
                                
                            
                        
                    
                    
                
                    
                        
                            zd1e - zd1e
                            
                                
                            
                        
                    
                    
                
                    
                        
                            zdlf - zdlf
                            
                                
                            
                        
                    
                    
                
                    
                        
                            五樓漫畫書專區 - 五樓漫畫書專區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            實體館藏 - 實體館藏
                            
                                
                            
                        
                    
                    
                
                    
                        
                            綜合書庫 - 綜合書庫
                            
                                
                            
                        
                    
                    
                
                    
                        
                            艾迪訊圖書館 - 艾迪訊圖書館
                            
                                
                            
                        
                    
                    
                
                    
                        
                            附中出版物專區 - 附中出版物專區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            龍華科技大學圖書館 - 龍華科技大學圖書館
                            
                                
                            
                        
                    
                    
                
                
            
                
                    確定
                    取消
                
                
                    
                        
                            B可借圖書
                            
                                
                            
                        
                    
                    
                
                    
                        
                            B電子資源
                            
                                
                            
                        
                    
                    
                
                    
                        
                            eee
                            
                                
                            
                        
                    
                    
                
                    
                        
                            M可借行動設備
                            
                                
                            
                        
                    
                    
                
                    
                        
                            P可借期刊
                            
                                
                            
                        
                    
                    
                
                    
                        
                            V可借視聽
                            
                                
                            
                        
                    
                    
                
                    
                        
                            www
                            
                                
                            
                        
                    
                    
                
                    
                        
                            不流通
                            
                                
                            
                        
                    
                    
                
                    
                        
                            書箱借閱30天
                            
                                
                            
                        
                    
                    
                
            
                
                    確定
                    取消
                
                
                    
                        
                            BD - 藍光光碟
                            
                                
                            
                        
                    
                    
                
                    
                        
                            CA - 靜畫資料
                            
                                
                            
                        
                    
                    
                
                    
                        
                            DB - 資料庫
                            
                                
                            
                        
                    
                    
                
                    
                        
                            DF - 磁片
                            
                                
                            
                        
                    
                    
                
                    
                        
                            DO - 電子書
                            
                                
                            
                        
                    
                    
                
                    
                        
                            EA - 立體模型
                            
                                
                            
                        
                    
                    
                
                    
                        
                            EB - 線上電子書
                            
                                
                            
                        
                    
                    
                
                    
                        
                            EJ - 線上電子期刊
                            
                                
                            
                        
                    
                    
                
                    
                        
                            EP - 電子期刊光碟
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ERROR - 有問題特藏
                            
                                
                            
                        
                    
                    
                
                    
                        
                            FA - 磁帶
                            
                                
                            
                        
                    
                    
                
                    
                        
                            KT - 多媒體組件
                            
                                
                            
                        
                    
                    
                
                    
                        
                            LA - 地圖
                            
                                
                            
                        
                    
                    
                
                    
                        
                            LD - 影碟
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MP - MP3
                            
                                
                            
                        
                    
                    
                
                    
                        
                            NH - 微縮單片
                            
                                
                            
                        
                    
                    
                
                    
                        
                            NR - 微縮捲片
                            
                                
                            
                        
                    
                    
                
                    
                        
                            QA - 地球儀
                            
                                
                            
                        
                    
                    
                
                    
                        
                            R - 參考書
                            
                                
                            
                        
                    
                    
                
                    
                        
                            SL - 幻燈片
                            
                                
                            
                        
                    
                    
                
                    
                        
                            VC - 錄影帶
                            
                                
                            
                        
                    
                    
                
                    
                        
                            VD - VCD
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BOX - 書箱
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ERM_DB - 電子資料庫(ERM)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ERM_WS - 網路資源(ERM)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ERM_EB - 電子書(ERM)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ERM_EJ - 電子期刊(ERM)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            XL - X-ray
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BOOK - 圖書
                            
                                
                            
                        
                    
                    
                
                    
                        
                            AC - 錄音帶
                            
                                
                            
                        
                    
                    
                
                    
                        
                            APP - 附件
                            
                                
                            
                        
                    
                    
                
                    
                        
                            P - 現期期刊
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ac - ac
                            
                                
                            
                        
                    
                    
                
                    
                        
                            DD - 影像光碟(DVD)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MD - 行動設備
                            
                                
                            
                        
                    
                    
                
                    
                        
                            S - 裝訂期刊
                            
                                
                            
                        
                    
                    
                
                    
                        
                            AD - 唱片
                            
                                
                            
                        
                    
                    
                
                    
                        
                            CD - 光碟
                            
                                
                            
                        
                    
                    
                
                    
                        
                            booklet - 小冊子
                            
                                
                            
                        
                    
                    
                
                    
                        
                            KKtest - KK
                            
                                
                            
                        
                    
                    
                
                    
                        
                            YYtest2 - YY2
                            
                                
                            
                        
                    
                    
                
                    
                        
                            YYtest4 - YYYtest
                            
                                
                            
                        
                    
                    
                
                    
                        
                            0425 - 0425
                            
                                
                            
                        
                    
                    
                
                    
                        
                            TEST - TEST
                            
                                
                            
                        
                    
                    
                
                    
                        
                            TEST0425 - TEST0425
                            
                                
                            
                        
                    
                    
                id(&quot;field2&quot;)</value>
      <webElementGuid>a331e243-914a-4ff5-a14b-14e7a9137cea</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;Body&quot;)</value>
      <webElementGuid>0e3308cc-d8df-48b2-8899-a80820fa10ba</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//body[@id='Body']</value>
      <webElementGuid>11a94227-5b62-4902-9917-e2f811687e15</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>b1a3f885-903e-4086-bc3e-f643325cbf08</webElementGuid>
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
				
				最近登入:2024-03-12 15:47:21 ,成功
				
				
					
					
                       0 筆新訊息
                    
                
				
				繁體中文English繁體中文
				
English
繁體中文
						
						
				
				
					
					登 出
				
			
		
		
		
			
			
				神資圖書館(15trunk機)
			
			
		
	
	
	
	
	
		
		
		
			
			
			 查詢  編目  流通  採購   期刊控制   場地設備  說明  我的帳戶  學科服務  管理  統計  清單  Portal  臉辨報表  ERM 
			簡單查詢進階查詢FRBR查詢最近的查詢已儲存的記錄FRBR作品書目書目記錄維護註記刪除之書目記錄書目記錄全域修改權威記錄權威記錄維護註記刪除之權威記錄權威記錄全域修改條碼號館藏資料維護註記刪除之館藏資料維護出版者出版者資料維護內部移送作業書單維護FRBR作品辦證讀者記錄維護檢查讀者註記刪除之讀者記錄讀者證卡記錄維護離職離校借閱檯讀者流通檯還書箱讀者查詢交易紀錄違規讀者查詢讀者個人違規記錄指定參考書交易歷史查詢一般搜尋交易館藏預約順位調整題名預約順位調整批次修改到期日內部移送作業盤點盤點維護跨館預約/調閱跨館預約/調閱需求移轉-寄送項目移轉-收到項目盤點比對批次報廢選擇報廢執行報廢已報廢歷史查詢書箱作業條碼號空號清單(條碼機印製)訂購檢查薦購請購訂購明細訂單登收用卷期登收程序發票款項維護預算經費供應商標案內部移送作業催缺候選催缺項目催缺記錄期刊主檔訂購檢查薦購請購核可訂購明細訂單登收用卷期登收程序發票款項維護預算經費供應商裝訂裝訂主檔維護擬裝設定送裝確認裝訂登收標案內部移送作業催缺候選催缺項目催缺記錄流通記錄預約記錄借用記錄維護分類維護單項維護參數設定流通政策維護預約審核場地設備黑名單書籤訊息排程工作日誌學科館藏館藏資料類型設定學科館藏維護學科預約教師資料維護讀者評價選單學科預約維護學科預約查詢教師評語學科報表學科館藏冊數統計教師學科服務時數與使用人次統計學科服務讀者清單學科報表學科館藏冊數統計教師學科服務時數與使用人次統計學科服務讀者清單一般設定類型/使用者(館員)片語設定系統設定貨幣匯率館藏地範本設定--依館員帳號翻譯轉換 選取清單格式屬性條碼號規範顯示設定圖書館系統屬性推薦範本設定館員新增申請QR Code 設定編目書目類型設定MARC規範格式設定顯示設定停用字SDI維護同義字條碼號原則對應流通流通通知單範本流通通知單參數設定流通通知單發送紀錄日常作業排程一般每周開閉館設定閉館日期維護交易紀錄類型交易方式可適用讀者身份交易方式可適用館藏流通類別交易方式可適用使用類別交易紀錄-館藏狀態流通政策讀者證卡原則單位所系夾書單欄位設定讀者閱讀興趣發送紀錄期刊週期傳閱清單維護 轉入Z39屬性設定更新書封上傳書封索引編修索引重建功能維護公佈欄維護意見回覆維護主題維護特別書籍維護心得分享東區互惠審核東區互惠匯出系統代碼維護貨幣代碼作者號維護科特號維護OPACSDI維護讀者評論管理Help維護熱門關鍵字過濾館員Log記錄館員登入/出記錄館員資料變更記錄讀者資料變更記錄權限設定MARC權限設定功能權限設定角色設定系統狀態排程管理排程監控者排程日誌查詢統計檢索點/檢索語統計報表不同時段使用次數統計書目下載次數統計電子資源統計電子資源-館藏點閱次數統計電子資源-讀者點閱次數統計電子資源-單位系所點閱次數統計編目統計館藏量統計報表類號區間統計報表圖書增減統計報表編目/館別/大類交叉分析報表大類/冊數統計表大類/特藏交叉統計表特藏/冊數統計表新書特藏大類交叉統計圖書增減表／月份／大類大類語言種數冊數統計大類/條碼號類別冊數統計圖書冊數分類統計單圖書冊數金額分類統計非書資料數量金額統計資料類型範圍館藏統計表編目工作量統計(個人)編目工作量統計(各館)館藏狀態統計報表流通統計讀者借閱排行榜書展借閱清單流通人數統計(學制分頁 符合科系)讀者類型借閱暨歸還流通量統計報表流通人次冊數統計  利用概況統計(日報表)讀者借閱交叉分析各館特藏預約統計報表館內使用統計讀者身份借閱統計讀者個人及單位所系借閱排行榜館藏資料借閱統計單位系所借閱排行榜單位系所借閱統計熱門借閱排行榜熱門預約排行榜讀者辦証暨補發人次統計表借閱交叉分析統計資料類型流通統計館員流通量統計單位(個人)保管圖書(非書)數量明細館際互借代還統計-本館館藏館際互借代還統計-他館館藏指定參考書借閱量統計表各大類熱門借閱排行榜大專院校圖書館讀者身份借閱清單及統計報表館藏流通量統計跨館預約/調閱館藏借閱排行榜罰款收款人數/次數統計跨館預約/調閱使用量統計罰款收款統計特定館藏借閱統計採購統計書商採購統計年度經費採購統計特藏訂購方式金額統計書商到書率書商績效統計期刊統計期刊到刊率統計報表平均收刊日統計報表歷年期刊借閱統計書商到書率書商催缺次數書商累計訂購次數期刊訂費漲幅統計逾期違約金統計期刊經費訂購統計報表場地設備場地設備使用統計表報表模組範本維護編目清單書目標籤報表Tag856檢查報表新版書標維護次分類交叉分析登錄簿條碼產生清單登錄簿清單條碼號空號清單單位科系館藏設定清單館藏狀態預約清單特殊館藏清單館藏狀態清單報表流通清單罰款未收款報表預約待取清單借閱(逾期)清單還書清單新書清單讀者欠款清單借閱逾期清單罰款收款清單預約撤架清單長期借閱圖書清單離線流通清單預約額滿資料清單單位系所借閱逾期清單跨館還書清單離職離校清單跨館移送清單預約保留結束清單指定參考書借閱量明細表未外借館藏清單重複讀者比對報表借閱逾期通知單取書櫃上架清單預約圖書清單採購清單年度經費採買書籍運用率訂購資料明細表擬購資料明細表請購單詢價單訂購逾期清單謝函書的催缺報表薦購書刊複本比對報表採購參考清單財產增加清冊財產減少清冊已付款未到清單訂購明細期刊清單期刊合訂本登錄簿交贈期刊清單期刊架位清單裝訂歷史清單薦購書刊複本比對報表期刊登錄簿索贈清單期刊查驗清單已付款未到清單場地設備預約取消清單臉辨用戶使用分析表臉辨用戶與圖書分類關係表期間辦證統計_年報期間辦證統計_月報期間辦證明細期間辦證異動統計_年報期間辦證異動統計_月報辦證異動明細臉辨使用紀錄統計_年報臉辨使用紀錄統計_月報臉辨使用紀錄明細人數統計表人數統計明細表人次統計表人次統計明細表熱門時段統計表讀者進出排行榜期間辦證刪除明細系統參數設定IP區間管理延伸查詢設定系統設定檔資源瀏覽/後分類設定資源到期通知設定可使用身分類別設定 代碼管理所屬資料庫清單共用代碼檔共用代碼類別電子資源管理資源前端顯示設定Ezproxy設定檔維護電子資料庫/網路資源電子期刊/電子書電子資源批次匯入電子資源副檔匯入讀者回報資源連線異常 Meniuri.erm.9000報表點閱資源排行榜依學院別使用統計依單位別使用統計依月分資源使用統計依學院登入統計表依系所登入統計表依身分登入統計表登入清單依身分別使用統計表使用者使用資源清單 收藏資源排行榜偵測電子資源狀態設定偵測資源時間設定偵測電子資源查詢電子資源狀態電子資源狀況報表
				
	


		
			
				編目 > 內部移送作業		
			
 	



	j(function () {
		j(&quot;div[id=&quot; , &quot;'&quot; , &quot;HiddenDiacritics&quot; , &quot;'&quot; , &quot;]&quot;).draggable({
  	      containment: &quot;#box&quot;,
 	       scroll: false
	});
	});
	j(function () {
		j(&quot;div[id=&quot; , &quot;'&quot; , &quot;hiddenDiacritics&quot; , &quot;'&quot; , &quot;]&quot;).draggable({
	        containment: &quot;#box&quot;,
	        scroll: false
	});
	});
 

	function syncWithOrder() {
		document.getElementById(&quot;orderCriteria&quot;).value = document
				.getElementById(&quot;browseCriteria&quot;).value;
	}
	function syncWithBrowse() {
		document.getElementById(&quot;browseCriteria&quot;).value = document
				.getElementById(&quot;orderCriteria&quot;).value;
	}

	var refreshTime = 0;

	function apasa() {
		refreshTime = 2500;
		clickLinkSubmit(&quot;searchForm&quot;, &quot;scriptSubmit&quot;);
	}

	function showingOrderBy(status) {
		document.getElementById(&quot;showOrderBy&quot;).style.display = status;
	}
	j(document).ready(function() {
		j(&quot;#resetbutton&quot;).click(function() {
			//j(&quot;#reseter&quot;).click();
			var h = j(&quot;#reseter&quot;).attr(&quot;href&quot;);
			window.location = h;
		});

	    // JSON 資料結構模擬 start
	    var treeJsonData = JSON.parse(j(&quot; , &quot;'&quot; , &quot;#locData&quot; , &quot;'&quot; , &quot;).text());
	    
	    j(&quot; , &quot;'&quot; , &quot;.place&quot; , &quot;'&quot; , &quot;).treeoptions({
	        data: treeJsonData,
	        openImg: &quot; , &quot;'&quot; , &quot;/inspireapp/images/ico_add.gif&quot; , &quot;'&quot; , &quot;, // img 路徑
	        cleanImg:&quot; , &quot;'&quot; , &quot;/inspireapp/images/clear.gif&quot; , &quot;'&quot; , &quot; // img 路徑
	    });
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
	});

	function createPopEdit(href) {

		popupwindow = window
				.open(
						&quot;&quot;,
						&quot;MeniuCatalogare&quot;,
						&quot;status=no,toolbar=0,scrollbars=yes,menubar=0,titlebar=0,resizable=1,width=1500,height=1500&quot;);
		popupwindow.moveTo(screen.width / 2 - 435, screen.height / 2 - 300);
		popupwindow.focus();

		popupwindow.location = href;

		if (popupwindow == null)
			popupwindow.opener = self;
		return false;

	}

	function createUploadPopEdit(href) {
		popupwindow = window.open(&quot;&quot;, &quot;Upload&quot;,
				&quot;status=no,toolbar=0,scrollbars=yes,menubar=0,titlebar=0,resizable=1,top=&quot;
						+ (screen.height - 150) / 2 + &quot;,left=&quot;
						+ (screen.width - 600) / 2 + &quot;,width=600,height=250&quot;);
		popupwindow.focus();

		popupwindow.location = href;

		if (popupwindow == null)
			popupwindow.opener = self;
		return false;
	}
	
	function runScript(e) {
	    if (e.keyCode == 13) {
	        document.getElementById(&quot;browse&quot;).click();
	        return false;
	    }
	}
	
	function changeModeDialog(msg, href){
		var crs = document.getElementsByClassName(&quot; , &quot;'&quot; , &quot;search_list_c&quot; , &quot;'&quot; , &quot;).length;
		
		if(crs){
			if(confirm(msg)){
				return createUploadPopEdit(href);
			}
			return false;
		}
		else{
			return createUploadPopEdit(href);
		}
	}



	  
		refreshTime = 0;
	
	







	











































		
			
				 
						 查詢
								
				   
						 條碼號輸入模式 
				
				  
				  上傳條碼號
				
			
		

		

		
			
				
					
						查詢條件
					
					
						
書名
出版者/書商/捐贈者
出版地
ISBN/ISSN
系統識別號
條碼號
其他號碼
索書號
分類號
館藏登收日期
館藏新增日期
館藏新增館員姓名
書目建立館員帳號
館藏修改日期
館藏修改館員姓名
館藏修改館員帳號
  
起始以
包含
等於(=)
大於(>=)
小於(&lt;=)
不等於(≠)

						 
								    
								
								
							 健康 ;健康100健康100 ;健康100 /健康101 : 101種天然食物提昇免疫力 /健康1本通 ;健康365 ;健康365 /健康4大基石 : 合理膳食, 適量運動, 戒煙限酒, 心理平衡. /健康6+1
								
								
							 
					
					
						
and
or
not
 
書名
出版者/書商/捐贈者
出版地
ISBN/ISSN
系統識別號
條碼號
其他號碼
索書號
分類號
館藏登收日期
館藏新增日期
館藏新增館員姓名
書目建立館員帳號
館藏修改日期
館藏修改館員姓名
館藏修改館員帳號
  
起始以
包含
等於(=)
大於(>=)
小於(&lt;=)
不等於(≠)

						 
								    
								
								
							 
								
								
							 
					
					
						
and
or
not
 
書名
出版者/書商/捐贈者
出版地
ISBN/ISSN
系統識別號
條碼號
其他號碼
索書號
分類號
館藏登收日期
館藏新增日期
館藏新增館員姓名
書目建立館員帳號
館藏修改日期
館藏修改館員姓名
館藏修改館員帳號
  
起始以
包含
等於(=)
大於(>=)
小於(&lt;=)
不等於(≠)

						  
								    
								
								
							
						
								
								
							
						
					
					
						
and
or
not
 
書名
出版者/書商/捐贈者
出版地
ISBN/ISSN
系統識別號
條碼號
其他號碼
索書號
分類號
館藏登收日期
館藏新增日期
館藏新增館員姓名
書目建立館員帳號
館藏修改日期
館藏修改館員姓名
館藏修改館員帳號
  
起始以
包含
等於(=)
大於(>=)
小於(&lt;=)
不等於(≠)

						  
								    
								
								
							
						
								
								
							
						
					

					
						
							 限制條件
						  
						
						
					
				
				
					
						
					
					
						
						
					
				
			

			
				
			

			
				
					{&quot;data&quot; :[{&quot;name&quot; : &quot;BD - 藍光光碟&quot;},{&quot;name&quot; : &quot;CA - 靜畫資料&quot;},{&quot;name&quot; : &quot;DB - 資料庫&quot;},{&quot;name&quot; : &quot;DF - 磁片&quot;},{&quot;name&quot; : &quot;DO - 電子書&quot;},{&quot;name&quot; : &quot;EA - 立體模型&quot;},{&quot;name&quot; : &quot;EB - 線上電子書&quot;},{&quot;name&quot; : &quot;EJ - 線上電子期刊&quot;},{&quot;name&quot; : &quot;EP - 電子期刊光碟&quot;},{&quot;name&quot; : &quot;ERROR - 有問題特藏&quot;},{&quot;name&quot; : &quot;FA - 磁帶&quot;},{&quot;name&quot; : &quot;KT - 多媒體組件&quot;},{&quot;name&quot; : &quot;LA - 地圖&quot;},{&quot;name&quot; : &quot;LD - 影碟&quot;},{&quot;name&quot; : &quot;MP - MP3&quot;},{&quot;name&quot; : &quot;NH - 微縮單片&quot;},{&quot;name&quot; : &quot;NR - 微縮捲片&quot;},{&quot;name&quot; : &quot;QA - 地球儀&quot;},{&quot;name&quot; : &quot;R - 參考書&quot;},{&quot;name&quot; : &quot;SL - 幻燈片&quot;},{&quot;name&quot; : &quot;VC - 錄影帶&quot;},{&quot;name&quot; : &quot;VD - VCD&quot;},{&quot;name&quot; : &quot;BOX - 書箱&quot;},{&quot;name&quot; : &quot;ERM_DB - 電子資料庫(ERM)&quot;},{&quot;name&quot; : &quot;ERM_WS - 網路資源(ERM)&quot;},{&quot;name&quot; : &quot;ERM_EB - 電子書(ERM)&quot;},{&quot;name&quot; : &quot;ERM_EJ - 電子期刊(ERM)&quot;},{&quot;name&quot; : &quot;XL - X-ray&quot;},{&quot;name&quot; : &quot;BOOK - 圖書&quot;},{&quot;name&quot; : &quot;AC - 錄音帶&quot;},{&quot;name&quot; : &quot;APP - 附件&quot;},{&quot;name&quot; : &quot;P - 現期期刊&quot;},{&quot;name&quot; : &quot;ac - ac&quot;},{&quot;name&quot; : &quot;DD - 影像光碟(DVD)&quot;},{&quot;name&quot; : &quot;MD - 行動設備&quot;},{&quot;name&quot; : &quot;S - 裝訂期刊&quot;},{&quot;name&quot; : &quot;AD - 唱片&quot;},{&quot;name&quot; : &quot;CD - 光碟&quot;},{&quot;name&quot; : &quot;booklet - 小冊子&quot;},{&quot;name&quot; : &quot;KKtest - KK&quot;},{&quot;name&quot; : &quot;YYtest2 - YY2&quot;},{&quot;name&quot; : &quot;YYtest4 - YYYtest&quot;},{&quot;name&quot; : &quot;0425 - 0425&quot;},{&quot;name&quot; : &quot;TEST - TEST&quot;},{&quot;name&quot; : &quot;TEST0425 - TEST0425&quot;}]}{&quot;data&quot; :[{&quot;name&quot; : &quot;B可借圖書&quot;},{&quot;name&quot; : &quot;B電子資源&quot;},{&quot;name&quot; : &quot;eee&quot;},{&quot;name&quot; : &quot;M可借行動設備&quot;},{&quot;name&quot; : &quot;P可借期刊&quot;},{&quot;name&quot; : &quot;V可借視聽&quot;},{&quot;name&quot; : &quot;www&quot;},{&quot;name&quot; : &quot;不流通&quot;},{&quot;name&quot; : &quot;書箱借閱30天&quot;}]}
						
							限制條件
							
						
						
							書目性質:
							
全系統記錄
圖書
期刊
分析款目
合集

						

						
						
						
						
						
							資料類型:
							
					   		
						


						
							館藏地:
							
								
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




								
						

						

						

						

						
						
						
						
							
							館藏狀態:
							
							
							

在架
借出
預約保留
遺失
盤點未到
聲明歸還
採購處理中
移送編目
編目處理中
移送閱覽
流通處理中
此筆盤點
核准跨館移送
等待送回原館藏地
等待轉送其他館藏地
跨館移送中
跨館轉送中
送回原館藏地
報銷中
報銷找回
重新歸架
展示中
已送裝訂
暫時不可提供
待報廢
已報廢
書箱借出
久借未還
尋書未獲
PickListValues.15004108
盤點結果狀態測試
測試選項
CWEN測試

							
						
						
						
							 
							 	館藏流通類別:
							 
							
							
							
						

						
							

							

							

							
							
						
					

				
			

		


		
			
				
					請指定要修改的資訊：
				
					
						館藏狀態
					

在架
盤點未到
採購處理中
移送編目
編目處理中
移送閱覽
流通處理中
此筆盤點
報銷中
報銷找回
重新歸架
展示中
暫時不可提供
久借未還
尋書未獲
PickListValues.15004108
盤點結果狀態測試
測試選項
CWEN測試

				
				
					
						在OPAC顯示
					

不在OPAC上顯示
在OPAC上顯示

					
				
				
					
						館藏地
					
					
							 
	function runScript(e) {
	    if (e.keyCode == 13) {
	        document.getElementById(&quot;browse&quot;).click();
	        return false;
	    }
	}				

 
 
 
  
  
   
  
  
  
  
   
  
  
   
    
   
   
館藏地CMUL - 神資圖書館2 - 2123 - 12320230417 - 2023041720230418 - 20230418AH - 安南醫院AHGL - 安南圖書區av - avB007 - B007BCSB4 - BCSB4BX - 取書櫃1BY - 取書櫃2CB - 北港分館BAVN - 北港分館視聽區(限館內閱覽)BCIR - 北港分館流通櫃檯BCRA - 北港分館指參(限館內閱覽)BPAV - 北港分館視聽區BPCL - 北港分館書庫cbook - cbookcircd - circdclp - clpCM - 北港附設醫院BMHL - 北港附設醫院圖書室CMUL - 中國醫藥大學圖書館 - CMUL - 中國醫藥大學圖書館CU - 台中總館CUAV - 台中總館視聽區(獨立專區)MAVN - 台中總館視聽區(限館內閱覽)MAVR - 台中總館視聽區MCAT - 台中總館技服組MCBS - 台中總館密閉書庫MCIR - 台中總館流通櫃檯MCRA - 台中總館教師指定參考書(限館內閱覽)MCSS - 台中總館B3裝訂期刊區MDIA - 台中總館博碩士論文區MEAS - 台中總館探索史懷哲之路專書區MEXM - 台中總館國考書區(限館內閱覽)MFGA - 台中總館本校教職優良教材區MFPA - 台中總館本校教師升等資料區MFSA - 台中總館本校教職論著MHME - 台中總館人文專書區-醫學倫理(限館內閱覽)台中總館人文專書區-醫學倫理(限館內閱覽)MHMH - 台中總館人文專書區-醫療史(限館內閱覽)MHML - 台中總館人文專書區-醫學法律(限館內閱覽)MHQC - 台中總館人文專書區-醫療品質(限館內閱覽)MNBR - 台中總館新書展示區MPAA - 台中總館績效暨獲獎區(限館內閱覽)MPBL - 台中總館PBL專書區(限館內閱覽)MREF - 台中總館參考室MRSS - 台中總館閱覽組MS-C - 台中總館期刊複本櫃MSER - 台中總館期刊區MSPA - 台中總館研究計劃專書MSPB - 中醫醫史文獻室(限所內閱覽)MSPC - 台中總館特藏室MSTK - 台中總館書庫MYBK - 台中總館參考壁櫃new item 7 - new item 7ONLN - 台中總館線上資料e-resources - 電子資源EB-P - EB-Pelect - electH-EQ - H-EQH-MR - H-MRL - LL40 - L40LB 圖書總館 - LB 圖書總館LB-S - LB-SLE - 語文教學中心LEGL - 語文教學中心圖書室LIB - LIBnew item 1 - new item 1new item 3 - new item 3new item 10 - new item 10new item 12 - new item 12new item 13 - new item 13new item 14 - new item 14new item 16 - new item 16new item 2 - new item 2new item 20 - new item 20new item 4 - new item 4new item 5 - new item 5new item 6 - 英才校區new item 8 - new item 8new item 9 - new item 9NPTU - NPTUOUK - OUKPT - 培德醫院new item 11 - new item 11new item 17 - new item 17PTGL - 培德醫院圖書區ptext - ptextSB3 - SB3T-P - T-PTBBK - TBBKTH - 台北分院THGL - 台北分院圖書區THPA - 台北分院期刊區W-P - W-PYH - 豐原分院new item 18 - new item 18new item 19 - new item 19YHGL - 豐原分院圖書區z3llc - z3llcz6bkf - z6bkfzd1a2 - zd1a2zd1e - zd1ezdlf - zdlf五樓漫畫書專區 - 五樓漫畫書專區實體館藏 - 實體館藏綜合書庫 - 綜合書庫艾迪訊圖書館 - 艾迪訊圖書館附中出版物專區 - 附中出版物專區龍華科技大學圖書館 - 龍華科技大學圖書館
  
  
window.dlocation = new dTree(&quot; , &quot;'&quot; , &quot;window.dlocation&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;messages&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;/inspireapp/images/&quot; , &quot;'&quot; , &quot;); 
window.dlocation.add(0,-1,&quot; , &quot;'&quot; , &quot;館藏地&quot; , &quot;'&quot; , &quot;); 
window.dlocation.add(1,0,&quot;CMUL - \u795E\u8CC7\u5716\u66F8\u9928&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;CMUL - \\u795E\\u8CC7\\u5716\\u66F8\\u9928&quot; , &quot;'&quot; , &quot;, 1, true)&quot;); 
window.dlocation.add(463,1,&quot;2 - 2&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;2 - 2&quot; , &quot;'&quot; , &quot;, 463, true)&quot;); 
window.dlocation.add(583,1,&quot;123 - 123&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;123 - 123&quot; , &quot;'&quot; , &quot;, 583, true)&quot;); 
window.dlocation.add(1183,1,&quot;20230417 - 20230417&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;20230417 - 20230417&quot; , &quot;'&quot; , &quot;, 1183, true)&quot;); 
window.dlocation.add(1203,1,&quot;20230418 - 20230418&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;20230418 - 20230418&quot; , &quot;'&quot; , &quot;, 1203, true)&quot;); 
window.dlocation.add(167,1,&quot;AH - \u5B89\u5357\u91AB\u9662&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;AH - \\u5B89\\u5357\\u91AB\\u9662&quot; , &quot;'&quot; , &quot;, 167, true)&quot;); 
window.dlocation.add(177,167,&quot;AHGL - \u5B89\u5357\u5716\u66F8\u5340&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;AHGL - \\u5B89\\u5357\\u5716\\u66F8\\u5340&quot; , &quot;'&quot; , &quot;, 177, true)&quot;); 
window.dlocation.add(643,1,&quot;av - av&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;av - av&quot; , &quot;'&quot; , &quot;, 643, true)&quot;); 
window.dlocation.add(303,1,&quot;B007 - B007&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;B007 - B007&quot; , &quot;'&quot; , &quot;, 303, true)&quot;); 
window.dlocation.add(883,1,&quot;BCSB4 - BCSB4&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;BCSB4 - BCSB4&quot; , &quot;'&quot; , &quot;, 883, true)&quot;); 
window.dlocation.add(823,1,&quot;BX - \u53D6\u66F8\u6AC31&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;BX - \\u53D6\\u66F8\\u6AC31&quot; , &quot;'&quot; , &quot;, 823, true)&quot;); 
window.dlocation.add(903,1,&quot;BY - \u53D6\u66F8\u6AC32&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;BY - \\u53D6\\u66F8\\u6AC32&quot; , &quot;'&quot; , &quot;, 903, true)&quot;); 
window.dlocation.add(169,1,&quot;CB - \u5317\u6E2F\u5206\u9928&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;CB - \\u5317\\u6E2F\\u5206\\u9928&quot; , &quot;'&quot; , &quot;, 169, true)&quot;); 
window.dlocation.add(179,169,&quot;BAVN - \u5317\u6E2F\u5206\u9928\u8996\u807D\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;BAVN - \\u5317\\u6E2F\\u5206\\u9928\\u8996\\u807D\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 179, true)&quot;); 
window.dlocation.add(180,169,&quot;BCIR - \u5317\u6E2F\u5206\u9928\u6D41\u901A\u6AC3\u6AAF&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;BCIR - \\u5317\\u6E2F\\u5206\\u9928\\u6D41\\u901A\\u6AC3\\u6AAF&quot; , &quot;'&quot; , &quot;, 180, true)&quot;); 
window.dlocation.add(181,169,&quot;BCRA - \u5317\u6E2F\u5206\u9928\u6307\u53C3(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;BCRA - \\u5317\\u6E2F\\u5206\\u9928\\u6307\\u53C3(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 181, true)&quot;); 
window.dlocation.add(182,169,&quot;BPAV - \u5317\u6E2F\u5206\u9928\u8996\u807D\u5340&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;BPAV - \\u5317\\u6E2F\\u5206\\u9928\\u8996\\u807D\\u5340&quot; , &quot;'&quot; , &quot;, 182, true)&quot;); 
window.dlocation.add(183,169,&quot;BPCL - \u5317\u6E2F\u5206\u9928\u66F8\u5EAB&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;BPCL - \\u5317\\u6E2F\\u5206\\u9928\\u66F8\\u5EAB&quot; , &quot;'&quot; , &quot;, 183, true)&quot;); 
window.dlocation.add(623,1,&quot;cbook - cbook&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;cbook - cbook&quot; , &quot;'&quot; , &quot;, 623, true)&quot;); 
window.dlocation.add(624,1,&quot;circd - circd&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;circd - circd&quot; , &quot;'&quot; , &quot;, 624, true)&quot;); 
window.dlocation.add(683,1,&quot;clp - clp&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;clp - clp&quot; , &quot;'&quot; , &quot;, 683, true)&quot;); 
window.dlocation.add(170,1,&quot;CM - \u5317\u6E2F\u9644\u8A2D\u91AB\u9662&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;CM - \\u5317\\u6E2F\\u9644\\u8A2D\\u91AB\\u9662&quot; , &quot;'&quot; , &quot;, 170, true)&quot;); 
window.dlocation.add(211,170,&quot;BMHL - \u5317\u6E2F\u9644\u8A2D\u91AB\u9662\u5716\u66F8\u5BA4&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;BMHL - \\u5317\\u6E2F\\u9644\\u8A2D\\u91AB\\u9662\\u5716\\u66F8\\u5BA4&quot; , &quot;'&quot; , &quot;, 211, true)&quot;); 
window.dlocation.add(363,1,&quot;CMUL - \u4E2D\u570B\u91AB\u85E5\u5927\u5B78\u5716\u66F8\u9928 - CMUL - \u4E2D\u570B\u91AB\u85E5\u5927\u5B78\u5716\u66F8\u9928&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;CMUL - \\u4E2D\\u570B\\u91AB\\u85E5\\u5927\\u5B78\\u5716\\u66F8\\u9928 - CMUL - \\u4E2D\\u570B\\u91AB\\u85E5\\u5927\\u5B78\\u5716\\u66F8\\u9928&quot; , &quot;'&quot; , &quot;, 363, true)&quot;); 
window.dlocation.add(171,1,&quot;CU - \u53F0\u4E2D\u7E3D\u9928&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;CU - \\u53F0\\u4E2D\\u7E3D\\u9928&quot; , &quot;'&quot; , &quot;, 171, true)&quot;); 
window.dlocation.add(603,171,&quot;CUAV - \u53F0\u4E2D\u7E3D\u9928\u8996\u807D\u5340(\u7368\u7ACB\u5C08\u5340)&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;CUAV - \\u53F0\\u4E2D\\u7E3D\\u9928\\u8996\\u807D\\u5340(\\u7368\\u7ACB\\u5C08\\u5340)&quot; , &quot;'&quot; , &quot;, 603, true)&quot;); 
window.dlocation.add(217,171,&quot;MAVN - \u53F0\u4E2D\u7E3D\u9928\u8996\u807D\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MAVN - \\u53F0\\u4E2D\\u7E3D\\u9928\\u8996\\u807D\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 217, true)&quot;); 
window.dlocation.add(218,171,&quot;MAVR - \u53F0\u4E2D\u7E3D\u9928\u8996\u807D\u5340&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MAVR - \\u53F0\\u4E2D\\u7E3D\\u9928\\u8996\\u807D\\u5340&quot; , &quot;'&quot; , &quot;, 218, true)&quot;); 
window.dlocation.add(219,171,&quot;MCAT - \u53F0\u4E2D\u7E3D\u9928\u6280\u670D\u7D44&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MCAT - \\u53F0\\u4E2D\\u7E3D\\u9928\\u6280\\u670D\\u7D44&quot; , &quot;'&quot; , &quot;, 219, true)&quot;); 
window.dlocation.add(220,171,&quot;MCBS - \u53F0\u4E2D\u7E3D\u9928\u5BC6\u9589\u66F8\u5EAB&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MCBS - \\u53F0\\u4E2D\\u7E3D\\u9928\\u5BC6\\u9589\\u66F8\\u5EAB&quot; , &quot;'&quot; , &quot;, 220, true)&quot;); 
window.dlocation.add(221,171,&quot;MCIR - \u53F0\u4E2D\u7E3D\u9928\u6D41\u901A\u6AC3\u6AAF&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MCIR - \\u53F0\\u4E2D\\u7E3D\\u9928\\u6D41\\u901A\\u6AC3\\u6AAF&quot; , &quot;'&quot; , &quot;, 221, true)&quot;); 
window.dlocation.add(244,171,&quot;MCRA - \u53F0\u4E2D\u7E3D\u9928\u6559\u5E2B\u6307\u5B9A\u53C3\u8003\u66F8(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MCRA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u6559\\u5E2B\\u6307\\u5B9A\\u53C3\\u8003\\u66F8(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 244, true)&quot;); 
window.dlocation.add(245,171,&quot;MCSS - \u53F0\u4E2D\u7E3D\u9928B3\u88DD\u8A02\u671F\u520A\u5340&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MCSS - \\u53F0\\u4E2D\\u7E3D\\u9928B3\\u88DD\\u8A02\\u671F\\u520A\\u5340&quot; , &quot;'&quot; , &quot;, 245, true)&quot;); 
window.dlocation.add(246,171,&quot;MDIA - \u53F0\u4E2D\u7E3D\u9928\u535A\u78A9\u58EB\u8AD6\u6587\u5340&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MDIA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u535A\\u78A9\\u58EB\\u8AD6\\u6587\\u5340&quot; , &quot;'&quot; , &quot;, 246, true)&quot;); 
window.dlocation.add(248,171,&quot;MEAS - \u53F0\u4E2D\u7E3D\u9928\u63A2\u7D22\u53F2\u61F7\u54F2\u4E4B\u8DEF\u5C08\u66F8\u5340&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MEAS - \\u53F0\\u4E2D\\u7E3D\\u9928\\u63A2\\u7D22\\u53F2\\u61F7\\u54F2\\u4E4B\\u8DEF\\u5C08\\u66F8\\u5340&quot; , &quot;'&quot; , &quot;, 248, true)&quot;); 
window.dlocation.add(250,171,&quot;MEXM - \u53F0\u4E2D\u7E3D\u9928\u570B\u8003\u66F8\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MEXM - \\u53F0\\u4E2D\\u7E3D\\u9928\\u570B\\u8003\\u66F8\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 250, true)&quot;); 
window.dlocation.add(251,171,&quot;MFGA - \u53F0\u4E2D\u7E3D\u9928\u672C\u6821\u6559\u8077\u512A\u826F\u6559\u6750\u5340&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MFGA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u672C\\u6821\\u6559\\u8077\\u512A\\u826F\\u6559\\u6750\\u5340&quot; , &quot;'&quot; , &quot;, 251, true)&quot;); 
window.dlocation.add(252,171,&quot;MFPA - \u53F0\u4E2D\u7E3D\u9928\u672C\u6821\u6559\u5E2B\u5347\u7B49\u8CC7\u6599\u5340&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MFPA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u672C\\u6821\\u6559\\u5E2B\\u5347\\u7B49\\u8CC7\\u6599\\u5340&quot; , &quot;'&quot; , &quot;, 252, true)&quot;); 
window.dlocation.add(253,171,&quot;MFSA - \u53F0\u4E2D\u7E3D\u9928\u672C\u6821\u6559\u8077\u8AD6\u8457&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MFSA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u672C\\u6821\\u6559\\u8077\\u8AD6\\u8457&quot; , &quot;'&quot; , &quot;, 253, true)&quot;); 
window.dlocation.add(256,171,&quot;MHME - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u5B78\u502B\u7406(\u9650\u9928\u5167\u95B1\u89BD)\u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u5B78\u502B\u7406(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MHME - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u5B78\\u502B\\u7406(\\u9650\\u9928\\u5167\\u95B1\\u89BD)\\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u5B78\\u502B\\u7406(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 256, true)&quot;); 
window.dlocation.add(257,171,&quot;MHMH - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u7642\u53F2(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MHMH - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u7642\\u53F2(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 257, true)&quot;); 
window.dlocation.add(258,171,&quot;MHML - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u5B78\u6CD5\u5F8B(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MHML - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u5B78\\u6CD5\\u5F8B(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 258, true)&quot;); 
window.dlocation.add(259,171,&quot;MHQC - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u7642\u54C1\u8CEA(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MHQC - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u7642\\u54C1\\u8CEA(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 259, true)&quot;); 
window.dlocation.add(261,171,&quot;MNBR - \u53F0\u4E2D\u7E3D\u9928\u65B0\u66F8\u5C55\u793A\u5340&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MNBR - \\u53F0\\u4E2D\\u7E3D\\u9928\\u65B0\\u66F8\\u5C55\\u793A\\u5340&quot; , &quot;'&quot; , &quot;, 261, true)&quot;); 
window.dlocation.add(262,171,&quot;MPAA - \u53F0\u4E2D\u7E3D\u9928\u7E3E\u6548\u66A8\u7372\u734E\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MPAA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7E3E\\u6548\\u66A8\\u7372\\u734E\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 262, true)&quot;); 
window.dlocation.add(263,171,&quot;MPBL - \u53F0\u4E2D\u7E3D\u9928PBL\u5C08\u66F8\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MPBL - \\u53F0\\u4E2D\\u7E3D\\u9928PBL\\u5C08\\u66F8\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 263, true)&quot;); 
window.dlocation.add(265,171,&quot;MREF - \u53F0\u4E2D\u7E3D\u9928\u53C3\u8003\u5BA4&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MREF - \\u53F0\\u4E2D\\u7E3D\\u9928\\u53C3\\u8003\\u5BA4&quot; , &quot;'&quot; , &quot;, 265, true)&quot;); 
window.dlocation.add(266,171,&quot;MRSS - \u53F0\u4E2D\u7E3D\u9928\u95B1\u89BD\u7D44&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MRSS - \\u53F0\\u4E2D\\u7E3D\\u9928\\u95B1\\u89BD\\u7D44&quot; , &quot;'&quot; , &quot;, 266, true)&quot;); 
window.dlocation.add(267,171,&quot;MS-C - \u53F0\u4E2D\u7E3D\u9928\u671F\u520A\u8907\u672C\u6AC3&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MS-C - \\u53F0\\u4E2D\\u7E3D\\u9928\\u671F\\u520A\\u8907\\u672C\\u6AC3&quot; , &quot;'&quot; , &quot;, 267, true)&quot;); 
window.dlocation.add(270,171,&quot;MSER - \u53F0\u4E2D\u7E3D\u9928\u671F\u520A\u5340&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MSER - \\u53F0\\u4E2D\\u7E3D\\u9928\\u671F\\u520A\\u5340&quot; , &quot;'&quot; , &quot;, 270, true)&quot;); 
window.dlocation.add(271,171,&quot;MSPA - \u53F0\u4E2D\u7E3D\u9928\u7814\u7A76\u8A08\u5283\u5C08\u66F8&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MSPA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7814\\u7A76\\u8A08\\u5283\\u5C08\\u66F8&quot; , &quot;'&quot; , &quot;, 271, true)&quot;); 
window.dlocation.add(272,171,&quot;MSPB - \u4E2D\u91AB\u91AB\u53F2\u6587\u737B\u5BA4(\u9650\u6240\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MSPB - \\u4E2D\\u91AB\\u91AB\\u53F2\\u6587\\u737B\\u5BA4(\\u9650\\u6240\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 272, true)&quot;); 
window.dlocation.add(273,171,&quot;MSPC - \u53F0\u4E2D\u7E3D\u9928\u7279\u85CF\u5BA4&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MSPC - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7279\\u85CF\\u5BA4&quot; , &quot;'&quot; , &quot;, 273, true)&quot;); 
window.dlocation.add(274,171,&quot;MSTK - \u53F0\u4E2D\u7E3D\u9928\u66F8\u5EAB&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MSTK - \\u53F0\\u4E2D\\u7E3D\\u9928\\u66F8\\u5EAB&quot; , &quot;'&quot; , &quot;, 274, true)&quot;); 
window.dlocation.add(275,171,&quot;MYBK - \u53F0\u4E2D\u7E3D\u9928\u53C3\u8003\u58C1\u6AC3&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MYBK - \\u53F0\\u4E2D\\u7E3D\\u9928\\u53C3\\u8003\\u58C1\\u6AC3&quot; , &quot;'&quot; , &quot;, 275, true)&quot;); 
window.dlocation.add(1103,171,&quot;new item 7 - new item 7&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;new item 7 - new item 7&quot; , &quot;'&quot; , &quot;, 1103, true)&quot;); 
window.dlocation.add(276,171,&quot;ONLN - \u53F0\u4E2D\u7E3D\u9928\u7DDA\u4E0A\u8CC7\u6599&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;ONLN - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7DDA\\u4E0A\\u8CC7\\u6599&quot; , &quot;'&quot; , &quot;, 276, true)&quot;); 
window.dlocation.add(3,1,&quot;e-resources - \u96FB\u5B50\u8CC7\u6E90&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;e-resources - \\u96FB\\u5B50\\u8CC7\\u6E90&quot; , &quot;'&quot; , &quot;, 3, true)&quot;); 
window.dlocation.add(345,1,&quot;EB-P - EB-P&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;EB-P - EB-P&quot; , &quot;'&quot; , &quot;, 345, true)&quot;); 
window.dlocation.add(648,1,&quot;elect - elect&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;elect - elect&quot; , &quot;'&quot; , &quot;, 648, true)&quot;); 
window.dlocation.add(343,1,&quot;H-EQ - H-EQ&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;H-EQ - H-EQ&quot; , &quot;'&quot; , &quot;, 343, true)&quot;); 
window.dlocation.add(344,1,&quot;H-MR - H-MR&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;H-MR - H-MR&quot; , &quot;'&quot; , &quot;, 344, true)&quot;); 
window.dlocation.add(543,1,&quot;L - L&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;L - L&quot; , &quot;'&quot; , &quot;, 543, true)&quot;); 
window.dlocation.add(863,1,&quot;L40 - L40&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;L40 - L40&quot; , &quot;'&quot; , &quot;, 863, true)&quot;); 
window.dlocation.add(1023,1,&quot;LB \u5716\u66F8\u7E3D\u9928 - LB \u5716\u66F8\u7E3D\u9928&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;LB \\u5716\\u66F8\\u7E3D\\u9928 - LB \\u5716\\u66F8\\u7E3D\\u9928&quot; , &quot;'&quot; , &quot;, 1023, true)&quot;); 
window.dlocation.add(323,1,&quot;LB-S - LB-S&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;LB-S - LB-S&quot; , &quot;'&quot; , &quot;, 323, true)&quot;); 
window.dlocation.add(173,1,&quot;LE - \u8A9E\u6587\u6559\u5B78\u4E2D\u5FC3&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;LE - \\u8A9E\\u6587\\u6559\\u5B78\\u4E2D\\u5FC3&quot; , &quot;'&quot; , &quot;, 173, true)&quot;); 
window.dlocation.add(280,173,&quot;LEGL - \u8A9E\u6587\u6559\u5B78\u4E2D\u5FC3\u5716\u66F8\u5BA4&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;LEGL - \\u8A9E\\u6587\\u6559\\u5B78\\u4E2D\\u5FC3\\u5716\\u66F8\\u5BA4&quot; , &quot;'&quot; , &quot;, 280, true)&quot;); 
window.dlocation.add(523,1,&quot;LIB - LIB&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;LIB - LIB&quot; , &quot;'&quot; , &quot;, 523, true)&quot;); 
window.dlocation.add(423,1,&quot;new item 1 - new item 1&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;new item 1 - new item 1&quot; , &quot;'&quot; , &quot;, 423, true)&quot;); 
window.dlocation.add(484,423,&quot;new item 3 - new item 3&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;new item 3 - new item 3&quot; , &quot;'&quot; , &quot;, 484, true)&quot;); 
window.dlocation.add(1283,1,&quot;new item 10 - new item 10&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;new item 10 - new item 10&quot; , &quot;'&quot; , &quot;, 1283, true)&quot;); 
window.dlocation.add(1323,1,&quot;new item 12 - new item 12&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;new item 12 - new item 12&quot; , &quot;'&quot; , &quot;, 1323, true)&quot;); 
window.dlocation.add(1343,1,&quot;new item 13 - new item 13&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;new item 13 - new item 13&quot; , &quot;'&quot; , &quot;, 1343, true)&quot;); 
window.dlocation.add(1344,1,&quot;new item 14 - new item 14&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;new item 14 - new item 14&quot; , &quot;'&quot; , &quot;, 1344, true)&quot;); 
window.dlocation.add(1264,1,&quot;new item 16 - new item 16&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;new item 16 - new item 16&quot; , &quot;'&quot; , &quot;, 1264, true)&quot;); 
window.dlocation.add(483,1,&quot;new item 2 - new item 2&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;new item 2 - new item 2&quot; , &quot;'&quot; , &quot;, 483, true)&quot;); 
window.dlocation.add(1425,1,&quot;new item 20 - new item 20&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;new item 20 - new item 20&quot; , &quot;'&quot; , &quot;, 1425, true)&quot;); 
window.dlocation.add(943,1,&quot;new item 4 - new item 4&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;new item 4 - new item 4&quot; , &quot;'&quot; , &quot;, 943, true)&quot;); 
window.dlocation.add(963,1,&quot;new item 5 - new item 5&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;new item 5 - new item 5&quot; , &quot;'&quot; , &quot;, 963, true)&quot;); 
window.dlocation.add(1063,1,&quot;new item 6 - \u82F1\u624D\u6821\u5340&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;new item 6 - \\u82F1\\u624D\\u6821\\u5340&quot; , &quot;'&quot; , &quot;, 1063, true)&quot;); 
window.dlocation.add(1243,1,&quot;new item 8 - new item 8&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;new item 8 - new item 8&quot; , &quot;'&quot; , &quot;, 1243, true)&quot;); 
window.dlocation.add(1263,1,&quot;new item 9 - new item 9&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;new item 9 - new item 9&quot; , &quot;'&quot; , &quot;, 1263, true)&quot;); 
window.dlocation.add(1043,1,&quot;NPTU - NPTU&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;NPTU - NPTU&quot; , &quot;'&quot; , &quot;, 1043, true)&quot;); 
window.dlocation.add(503,1,&quot;OUK - OUK&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;OUK - OUK&quot; , &quot;'&quot; , &quot;, 503, true)&quot;); 
window.dlocation.add(174,1,&quot;PT - \u57F9\u5FB7\u91AB\u9662&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;PT - \\u57F9\\u5FB7\\u91AB\\u9662&quot; , &quot;'&quot; , &quot;, 174, true)&quot;); 
window.dlocation.add(1303,174,&quot;new item 11 - new item 11&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;new item 11 - new item 11&quot; , &quot;'&quot; , &quot;, 1303, true)&quot;); 
window.dlocation.add(1363,174,&quot;new item 17 - new item 17&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;new item 17 - new item 17&quot; , &quot;'&quot; , &quot;, 1363, true)&quot;); 
window.dlocation.add(283,174,&quot;PTGL - \u57F9\u5FB7\u91AB\u9662\u5716\u66F8\u5340&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;PTGL - \\u57F9\\u5FB7\\u91AB\\u9662\\u5716\\u66F8\\u5340&quot; , &quot;'&quot; , &quot;, 283, true)&quot;); 
window.dlocation.add(645,1,&quot;ptext - ptext&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;ptext - ptext&quot; , &quot;'&quot; , &quot;, 645, true)&quot;); 
window.dlocation.add(1083,1,&quot;SB3 - SB3&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;SB3 - SB3&quot; , &quot;'&quot; , &quot;, 1083, true)&quot;); 
window.dlocation.add(324,1,&quot;T-P - T-P&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;T-P - T-P&quot; , &quot;'&quot; , &quot;, 324, true)&quot;); 
window.dlocation.add(1403,1,&quot;TBBK - TBBK&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;TBBK - TBBK&quot; , &quot;'&quot; , &quot;, 1403, true)&quot;); 
window.dlocation.add(175,1,&quot;TH - \u53F0\u5317\u5206\u9662&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;TH - \\u53F0\\u5317\\u5206\\u9662&quot; , &quot;'&quot; , &quot;, 175, true)&quot;); 
window.dlocation.add(284,175,&quot;THGL - \u53F0\u5317\u5206\u9662\u5716\u66F8\u5340&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;THGL - \\u53F0\\u5317\\u5206\\u9662\\u5716\\u66F8\\u5340&quot; , &quot;'&quot; , &quot;, 284, true)&quot;); 
window.dlocation.add(285,175,&quot;THPA - \u53F0\u5317\u5206\u9662\u671F\u520A\u5340&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;THPA - \\u53F0\\u5317\\u5206\\u9662\\u671F\\u520A\\u5340&quot; , &quot;'&quot; , &quot;, 285, true)&quot;); 
window.dlocation.add(325,1,&quot;W-P - W-P&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;W-P - W-P&quot; , &quot;'&quot; , &quot;, 325, true)&quot;); 
window.dlocation.add(176,1,&quot;YH - \u8C50\u539F\u5206\u9662&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;YH - \\u8C50\\u539F\\u5206\\u9662&quot; , &quot;'&quot; , &quot;, 176, true)&quot;); 
window.dlocation.add(1423,176,&quot;new item 18 - new item 18&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;new item 18 - new item 18&quot; , &quot;'&quot; , &quot;, 1423, true)&quot;); 
window.dlocation.add(1424,176,&quot;new item 19 - new item 19&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;new item 19 - new item 19&quot; , &quot;'&quot; , &quot;, 1424, true)&quot;); 
window.dlocation.add(286,176,&quot;YHGL - \u8C50\u539F\u5206\u9662\u5716\u66F8\u5340&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;YHGL - \\u8C50\\u539F\\u5206\\u9662\\u5716\\u66F8\\u5340&quot; , &quot;'&quot; , &quot;, 286, true)&quot;); 
window.dlocation.add(983,1,&quot;z3llc - z3llc&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;z3llc - z3llc&quot; , &quot;'&quot; , &quot;, 983, true)&quot;); 
window.dlocation.add(647,1,&quot;z6bkf - z6bkf&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;z6bkf - z6bkf&quot; , &quot;'&quot; , &quot;, 647, true)&quot;); 
window.dlocation.add(646,1,&quot;zd1a2 - zd1a2&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;zd1a2 - zd1a2&quot; , &quot;'&quot; , &quot;, 646, true)&quot;); 
window.dlocation.add(663,1,&quot;zd1e - zd1e&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;zd1e - zd1e&quot; , &quot;'&quot; , &quot;, 663, true)&quot;); 
window.dlocation.add(644,1,&quot;zdlf - zdlf&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;zdlf - zdlf&quot; , &quot;'&quot; , &quot;, 644, true)&quot;); 
window.dlocation.add(403,1,&quot;\u4E94\u6A13\u6F2B\u756B\u66F8\u5C08\u5340 - \u4E94\u6A13\u6F2B\u756B\u66F8\u5C08\u5340&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E94\\u6A13\\u6F2B\\u756B\\u66F8\\u5C08\\u5340 - \\u4E94\\u6A13\\u6F2B\\u756B\\u66F8\\u5C08\\u5340&quot; , &quot;'&quot; , &quot;, 403, true)&quot;); 
window.dlocation.add(563,1,&quot;\u5BE6\u9AD4\u9928\u85CF - \u5BE6\u9AD4\u9928\u85CF&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;\\u5BE6\\u9AD4\\u9928\\u85CF - \\u5BE6\\u9AD4\\u9928\\u85CF&quot; , &quot;'&quot; , &quot;, 563, true)&quot;); 
window.dlocation.add(1383,1,&quot;\u7D9C\u5408\u66F8\u5EAB - \u7D9C\u5408\u66F8\u5EAB&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;\\u7D9C\\u5408\\u66F8\\u5EAB - \\u7D9C\\u5408\\u66F8\\u5EAB&quot; , &quot;'&quot; , &quot;, 1383, true)&quot;); 
window.dlocation.add(383,1,&quot;\u827E\u8FEA\u8A0A\u5716\u66F8\u9928 - \u827E\u8FEA\u8A0A\u5716\u66F8\u9928&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;\\u827E\\u8FEA\\u8A0A\\u5716\\u66F8\\u9928 - \\u827E\\u8FEA\\u8A0A\\u5716\\u66F8\\u9928&quot; , &quot;'&quot; , &quot;, 383, true)&quot;); 
window.dlocation.add(1384,1,&quot;\u9644\u4E2D\u51FA\u7248\u7269\u5C08\u5340 - \u9644\u4E2D\u51FA\u7248\u7269\u5C08\u5340&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;\\u9644\\u4E2D\\u51FA\\u7248\\u7269\\u5C08\\u5340 - \\u9644\\u4E2D\\u51FA\\u7248\\u7269\\u5C08\\u5340&quot; , &quot;'&quot; , &quot;, 1384, true)&quot;); 
window.dlocation.add(443,1,&quot;\u9F8D\u83EF\u79D1\u6280\u5927\u5B78\u5716\u66F8\u9928 - \u9F8D\u83EF\u79D1\u6280\u5927\u5B78\u5716\u66F8\u9928&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;\\u9F8D\\u83EF\\u79D1\\u6280\\u5927\\u5B78\\u5716\\u66F8\\u9928 - \\u9F8D\\u83EF\\u79D1\\u6280\\u5927\\u5B78\\u5716\\u66F8\\u9928&quot; , &quot;'&quot; , &quot;, 443, true)&quot;); 
window.dlocation.selectElement = function(lname, id, hideTree) { 
document.getElementById(&quot; , &quot;'&quot; , &quot;location_0&quot; , &quot;'&quot; , &quot;).value = id; 
document.getElementById(&quot; , &quot;'&quot; , &quot;elementName&quot; , &quot;'&quot; , &quot;).value = lname; 
if(hideTree == true) changeStatus(&quot; , &quot;'&quot; , &quot;locationTree&quot; , &quot;'&quot; , &quot;); 
}; 
 document.getElementById(&quot; , &quot;'&quot; , &quot;locationArea&quot; , &quot;'&quot; , &quot;).innerHTML =  window.dlocation; 
  
  
  


						
				
			
			
				
					
						請輸入要修改的條碼號
						
條碼號

						
							
							

//&lt;![CDATA[

								document.getElementById(&quot;listField&quot;).focus();
							
//]]&gt;


						
						
						
						
					
				
			
		

		
			
				注意:若是修改的資料內容與原始內容相同，則不修改亦不顯示。
			

			
		
		

			
				
					
						
							
								
									
										排序條件:   
條碼號
分類號

										
									
								
								
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
500
 
			
		

		
			

        	

	






            




	jQuery.noConflict();
	var popNewBookSelect={
	          node_this:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,
			  code:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,       
			  name:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,
			  popUp : function(codePtr,namePtr,top,left,obj){//POPUP WINDOW------------------------------------------------------
			          (function($){
						  $(&quot;#popNewBookSelect&quot;).css({&quot; , &quot;'&quot; , &quot;top&quot; , &quot;'&quot; , &quot; : top ,&quot; , &quot;'&quot; , &quot;left&quot; , &quot;'&quot; , &quot; : left});//位置設定
						  popNewBookSelect.node_this  = $(obj);
						  popNewBookSelect.code  	   = codePtr;
						  popNewBookSelect.name  	   = namePtr;
				          //var dropdownSet = $(&quot; , &quot;'&quot; , &quot;#popNewBookSelect #L1dropdown&quot; , &quot;'&quot; , &quot;);
		                  popNewBookSelect.init();
					  })(jQuery);
			  },
              init : function(){
			          (function($){
						    var contextPath=trim($(&quot; , &quot;'&quot; , &quot;#popNewBookSelect #contextPath&quot; , &quot;'&quot; , &quot;).text());//取contextPath
			                $(&quot; , &quot;'&quot; , &quot;#popNewBookSelect img.clear&quot; , &quot;'&quot; , &quot;).attr(&quot;src&quot;,contextPath+&quot;images/clear.gif&quot;);
			                $(&quot; , &quot;'&quot; , &quot;#popNewBookSelect img.wait&quot; , &quot;'&quot; , &quot;).attr(&quot;src&quot;,contextPath+&quot;images/wait.gif&quot;);
			                //alert($(&quot; , &quot;'&quot; , &quot;#popNewBookSelect img.clear&quot; , &quot;'&quot; , &quot;).attr(&quot;src&quot;));
			                $(&quot;#popNewBookSelect input.button&quot;).val($(&quot; , &quot;'&quot; , &quot;#popNewBookSelect #save&quot; , &quot;'&quot; , &quot;).text());
			              	$(&quot;#popNewBookSelect&quot;).show();
					  })(jQuery);
              },
		      cancel : function() {//CANCEL----------------------------------------------------------------------------
					(function($){
							$(&quot;#popNewBookSelect&quot;).hide();
					})(jQuery);
			  },
		      checkBoxClear : function() {//checkBox Clear----------------------------------------------------------------------------
					(function($){
						$(&quot;#JQ_ItemCRS input.check&quot;).each(function(){
								$(this).attr(&quot;checked&quot;,false);
						});
						$(&quot;#rootCheck&quot;).attr(&quot;checked&quot;,false);
					})(jQuery);
			  },
		      confirm : function() {//run--------------------------------------------------------------------------
					(function($){
						$(&quot;#popNewBookSelect&quot;).hide();
						var contextPath=trim($(&quot; , &quot;'&quot; , &quot;#popNewBookSelect #contextPath&quot; , &quot;'&quot; , &quot;).text());//取contextPath
						var year =$(&quot; , &quot;'&quot; , &quot;#year  :selected&quot; , &quot;'&quot; , &quot;).text();
						var month=$(&quot; , &quot;'&quot; , &quot;#month :selected&quot; , &quot;'&quot; , &quot;).text();
					    var i=0;
					    var flag=&quot;n&quot;;
					    var selectItems=&quot;&quot;;
						$(&quot;#JQ_ItemCRS input.check&quot;).each(function(){
								//alert(i+&quot; - &quot;+$(this).attr(&quot;checked&quot;));
								var select=$(this).attr(&quot;checked&quot;);
								if(select==true){
									//alert(i+&quot; - &quot;+$(this).parent().parent().find(&quot;td[className=itemId]&quot;).text());
									var itemId=trim($(this).parent().parent().find(&quot;td[className=itemId]&quot;).text());
									if(flag==&quot;y&quot;) selectItems=selectItems+&quot;,&quot;;
									selectItems=selectItems+itemId;
									flag=&quot;y&quot;;
								}
								i++;
						});
						//alert(&quot;REQ=&quot;+selectItems);
						//alert(&quot;year=&quot;+year);
						//alert(&quot;month=&quot;+month);
						popNewBookSelect.checkBoxClear();
				        $.ajax({ 
							    type: &quot;POST&quot;, 
							    url: contextPath+&quot;/kidMain&quot;, 
							    dataType: &quot;text&quot;, 
							    data: {method:&quot;setNewBooks&quot;,selectItems:selectItems,year:year,month:month}, 
							    success: function(html) {
										//alert(&quot;RES=&quot;+html);
										var success=$(&quot; , &quot;'&quot; , &quot;#popNewBookSelect #success&quot; , &quot;'&quot; , &quot;).text();
										alert(success);
							    }, 
							    error: function (XMLHttpRequest, textStatus, errorThrown) { 
							    	alert(XMLHttpRequest.responseText);
							    } 
						});
					})(jQuery);
			  }
	}	  


&lt;!--
	#popNewBookSelect {
		position: absolute;
		left: 250px;
		top: 200px;
		z-index: 1;
	}
	#popNewBookSelect .pop_foreground {
	z-index:3;
	left: 21px;
	background-color: #FFFFFF;
	position: absolute;
	width: 342px;
	border-top-width: 1px;
	border-right-width: 1px;
	border-bottom-width: 1px;
	border-left-width: 1px;
	border-top-style: solid;
	border-right-style: solid;
	border-bottom-style: solid;
	border-left-style: solid;
	border-top-color: #84A9D8;
	border-right-color: #84A9D8;
	border-bottom-color: #84A9D8;
	border-left-color: #84A9D8;
	}
	#popNewBookSelect .pop_foreground .pop_cancel {
		position: absolute;
		top: 6px;
		right: 10px;
	}
	#popNewBookSelect .pop_foreground .pop_cancel a {
		color: #FF0000;
		text-decoration: none;
	}
	#popNewBookSelect .pop_foreground  .pop_header {
	color: #FFFFFF;
	background-color: #84A9D8;
	margin-top: 0px;
	margin-bottom: 0px;
	}
	#popNewBookSelect .pop_foreground  .pop_header p {
		margin-top: 0px;
		margin-bottom: 0px;
		font-size: 12pt;
		line-height: 2em;
		font-weight: bold;
	}
	#popNewBookSelect .pop_foreground .pop_content {
		width: 100%;
		margin-top: 0px;
		margin-bottom: 0px;
		border: 1px solid #666666;
		background: #FFFFFF;
	}
	#popNewBookSelect .pop_foreground .pop_content a {
		text-decoration: none;
		font-size: 12pt;
		line-height: 1em;
		color: #000000;
		display: block;
	}
	#popNewBookSelect .pop_foreground .pop_content a:hover {
		text-decoration: none;
		font-size: 12pt;
		line-height: 1em;
		color: #FFFFFF;
		background: #a69f00;
	}
	#popNewBookSelect .pop_foreground  .pop_content p {
		margin-top: 0px;
		margin-bottom: 0px;
		font-size: 12pt;
		line-height: 1em;
	}
	#popNewBookSelect .pop_foreground .pop_footer {
		background: #FFA100;
		text-align: center;
		font-size: 12pt;
		line-height: 1em;
		color: #993300;
	}

	#popNewBookSelect #detailFormContainer {
	width: 322px;
	padding-top: 2px;
	padding-right: 10px;
	padding-bottom: 12px;
	padding-left: 10px;
	}
	#popNewBookSelect #detailFormContainer br {
			clear: both;
	}
	#popNewBookSelect #detailFormContainer #cascadingDropdowns div {
		  float: left;
		  margin-right: 10px;
	}
	#popNewBookSelect #detailFormContainer label {
			float: left;
			margin-right: 10px;
			color: #FFFFFF;
			font: 24px &quot;標楷體&quot;;
	}
#popNewBookSelect #confirm {
	text-align: right;
	margin-left: 225px;
	margin-top: 20px;
}
#popNewBookSelect #confirm .button {
	background-color: #84A9D8;
	margin: 0px;
	border: 1px outset #CCC;
}
-->



		/inspireapp/
		
		
	    
	    	
	        
	          	
	        
	        
      			
                    
                    
                          
                          	
							      
									  
									        2012
									        2011
									        2010
									        2009
									        2008
									        2007
									        2006
									        2005
									        2004
									        2003
									        2002
									        2001
									        2000
								      
							      
							           
									  
									          01
									          02
									          03
									          04
									          05
									          06
									          07
									          08
									          09
									          10
									          11
									          12
								      
							      
							  
	                          
 		    						
							  
                          
                    
                    
                
            
	    

	
	

        	




	jQuery.noConflict();
	var popGoodBookSelect={
	          node_this:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,
			  code:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,       
			  name:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,
			  popUp : function(codePtr,namePtr,top,left,obj){//POPUP WINDOW------------------------------------------------------
			          (function($){
						  $(&quot;#popGoodBookSelect&quot;).css({&quot; , &quot;'&quot; , &quot;top&quot; , &quot;'&quot; , &quot; : top ,&quot; , &quot;'&quot; , &quot;left&quot; , &quot;'&quot; , &quot; : left});//位置設定
						  popGoodBookSelect.node_this  = $(obj);
						  popGoodBookSelect.code  	   = codePtr;
						  popGoodBookSelect.name  	   = namePtr;
		                  popGoodBookSelect.init();
					  })(jQuery);
			  },
              init : function(){
			          (function($){
						    var contextPath=trim($(&quot; , &quot;'&quot; , &quot;#popGoodBookSelect #contextPath&quot; , &quot;'&quot; , &quot;).text());//取contextPath
			                $(&quot; , &quot;'&quot; , &quot;#popGoodBookSelect img.clear&quot; , &quot;'&quot; , &quot;).attr(&quot;src&quot;,contextPath+&quot;images/clear.gif&quot;);
			                $(&quot; , &quot;'&quot; , &quot;#popGoodBookSelect img.wait&quot; , &quot;'&quot; , &quot;).attr(&quot;src&quot;,contextPath+&quot;images/wait.gif&quot;);
					        $.ajax({ 
								    type: &quot;POST&quot;, 
								    url: contextPath+&quot;/kidMain&quot;, 
								    dataType: &quot;json&quot;, 
								    data: {method:&quot;getGoodBookItemCode&quot;,language:&quot;4&quot;,ignore:&quot;00&quot;}, 
								    success: function(data) {
										  //alert(&quot;RES=&quot;+data);
						              	  $(&quot;#popGoodBookSelect #L1DropDown&quot;).loadSelect(data);
								    }, 
								    error: function (XMLHttpRequest, textStatus, errorThrown) { 
								    	alert(XMLHttpRequest.responseText);
								    } 
							});
			                $(&quot;#popGoodBookSelect input.button&quot;).val($(&quot; , &quot;'&quot; , &quot;#popGoodBookSelect #save&quot; , &quot;'&quot; , &quot;).text());
			              	$(&quot;#popGoodBookSelect&quot;).show();
					  })(jQuery);
              },
		      cancel : function() {//CANCEL----------------------------------------------------------------------------
					(function($){
							$(&quot;#popGoodBookSelect&quot;).hide();
					})(jQuery);
			  },
		      checkBoxClear : function() {//checkBox Clear----------------------------------------------------------------------------
					(function($){
						$(&quot;#JQ_ItemCRS input.check&quot;).each(function(){
								$(this).attr(&quot;checked&quot;,false);
						});
						$(&quot;#rootCheck&quot;).attr(&quot;checked&quot;,false);
					})(jQuery);
			  },
		      confirm : function() {//run--------------------------------------------------------------------------
					(function($){
						$(&quot;#popGoodBookSelect&quot;).hide();
						var contextPath=trim($(&quot; , &quot;'&quot; , &quot;#popGoodBookSelect #contextPath&quot; , &quot;'&quot; , &quot;).text());//取contextPath
						//var id =$(&quot; , &quot;'&quot; , &quot;#popGoodBookSelect #topicId  :selected&quot; , &quot;'&quot; , &quot;).val();
						var id=$(&quot; , &quot;'&quot; , &quot;#popGoodBookSelect #L1DropDown&quot; , &quot;'&quot; , &quot;).val();
					    var i=0;
					    var flag=&quot;n&quot;;
					    var selectItems=&quot;&quot;;
						$(&quot;#JQ_ItemCRS input.check&quot;).each(function(){
								//alert(i+&quot; - &quot;+$(this).attr(&quot;checked&quot;));
								var select=$(this).attr(&quot;checked&quot;);
								if(select==true){
									//alert(i+&quot; - &quot;+$(this).parent().parent().find(&quot;td[className=itemId]&quot;).text());
									var itemId=trim($(this).parent().parent().find(&quot;td[className=itemId]&quot;).text());
									if(flag==&quot;y&quot;) selectItems=selectItems+&quot;,&quot;;
									selectItems=selectItems+itemId;
									flag=&quot;y&quot;;
								}
								i++;
						});
						//alert(&quot;good book REQ=&quot;+selectItems);
						//alert(&quot;good book topc-id=&quot;+id);
						popGoodBookSelect.checkBoxClear();
				        $.ajax({ 
							    type: &quot;POST&quot;, 
							    url: contextPath+&quot;/kidMain&quot;, 
							    dataType: &quot;text&quot;, 
							    data: {method:&quot;setGoodBooks&quot;,selectItems:selectItems,id:id}, 
							    success: function(html) {
										//alert(&quot;RES=&quot;+html);
										var success=$(&quot; , &quot;'&quot; , &quot;#popGoodBookSelect #success&quot; , &quot;'&quot; , &quot;).text();
										alert(success);
							    }, 
							    error: function (XMLHttpRequest, textStatus, errorThrown) { 
							    	alert(XMLHttpRequest.responseText);
							    } 
						});
					})(jQuery);
			  }
	}	  


&lt;!--
	#popGoodBookSelect {
		position: absolute;
		left: 250px;
		top: 200px;
		z-index: 1;
	}
	#popGoodBookSelect .pop_foreground {
	z-index:3;
	left: 21px;
	background-color: #FFFFFF;
	position: absolute;
	width: 342px;
	border-top-width: 1px;
	border-right-width: 1px;
	border-bottom-width: 1px;
	border-left-width: 1px;
	border-top-style: solid;
	border-right-style: solid;
	border-bottom-style: solid;
	border-left-style: solid;
	border-top-color: #84A9D8;
	border-right-color: #84A9D8;
	border-bottom-color: #84A9D8;
	border-left-color: #84A9D8;
	}
	#popGoodBookSelect .pop_foreground .pop_cancel {
		position: absolute;
		top: 6px;
		right: 10px;
	}
	#popGoodBookSelect .pop_foreground .pop_cancel a {
		color: #FF0000;
		text-decoration: none;
	}
	#popGoodBookSelect .pop_foreground  .pop_header {
	color: #FFFFFF;
	background-color: #84A9D8;
	margin-top: 0px;
	margin-bottom: 0px;
	}
	#popGoodBookSelect .pop_foreground  .pop_header p {
		margin-top: 0px;
		margin-bottom: 0px;
		font-size: 12pt;
		line-height: 2em;
		font-weight: bold;
	}
	#popGoodBookSelect .pop_foreground .pop_content {
		width: 100%;
		margin-top: 0px;
		margin-bottom: 0px;
		border: 1px solid #666666;
		background: #FFFFFF;
	}
	#popGoodBookSelect .pop_foreground .pop_content a {
		text-decoration: none;
		font-size: 12pt;
		line-height: 1em;
		color: #000000;
		display: block;
	}
	#popGoodBookSelect .pop_foreground .pop_content a:hover {
		text-decoration: none;
		font-size: 12pt;
		line-height: 1em;
		color: #FFFFFF;
		background: #a69f00;
	}
	#popGoodBookSelect .pop_foreground  .pop_content p {
		margin-top: 0px;
		margin-bottom: 0px;
		font-size: 12pt;
		line-height: 1em;
	}
	#popGoodBookSelect .pop_foreground .pop_footer {
		background: #FFA100;
		text-align: center;
		font-size: 12pt;
		line-height: 1em;
		color: #993300;
	}

	#popGoodBookSelect #detailFormContainer {
	width: 322px;
	padding-top: 2px;
	padding-right: 10px;
	padding-bottom: 12px;
	padding-left: 10px;
	}
	#popGoodBookSelect #detailFormContainer br {
			clear: both;
	}
	#popGoodBookSelect #detailFormContainer #cascadingDropdowns div {
		  float: left;
		  margin-right: 10px;
	}
	#popGoodBookSelect #detailFormContainer label {
			float: left;
			margin-right: 10px;
			color: #FFFFFF;
			font: 24px &quot;標楷體&quot;;
	}
#popGoodBookSelect #confirm {
	text-align: right;
	margin-left: 225px;
	margin-top: 20px;
}
#popGoodBookSelect #confirm .button {
	background-color: #84A9D8;
	margin: 0px;
	border: 1px outset #CCC;
}
-->



		/inspireapp/
		
		
	    
	    	
	        
	          	Good Book Setup
	        
	        
      			
                    
                    
                          
                          	  
							      
	                              
							      
							  
	                          
 		    						
							  
                          
                    
                    
                
            
	    

	
	

        	




	jQuery.noConflict();
	var popTopicBookSelect={
	          node_this:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,
			  code:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,       
			  name:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,
			  popUp : function(codePtr,namePtr,top,left,obj){//POPUP WINDOW------------------------------------------------------
			          (function($){
						  $(&quot;#popTopicBookSelect&quot;).css({&quot; , &quot;'&quot; , &quot;top&quot; , &quot;'&quot; , &quot; : top ,&quot; , &quot;'&quot; , &quot;left&quot; , &quot;'&quot; , &quot; : left});//位置設定
						  popTopicBookSelect.node_this  = $(obj);
						  popTopicBookSelect.code  	   = codePtr;
						  popTopicBookSelect.name  	   = namePtr;
		                  popTopicBookSelect.init();
					  })(jQuery);
			  },
              init : function(){
			          (function($){
						    var contextPath=trim($(&quot; , &quot;'&quot; , &quot;#popTopicBookSelect #contextPath&quot; , &quot;'&quot; , &quot;).text());//取contextPath
			                $(&quot; , &quot;'&quot; , &quot;#popTopicBookSelect img.clear&quot; , &quot;'&quot; , &quot;).attr(&quot;src&quot;,contextPath+&quot;images/clear.gif&quot;);
			                $(&quot; , &quot;'&quot; , &quot;#popTopicBookSelect img.wait&quot; , &quot;'&quot; , &quot;).attr(&quot;src&quot;,contextPath+&quot;images/wait.gif&quot;);
					        $.ajax({ 
								    type: &quot;POST&quot;, 
								    url: contextPath+&quot;/kidMain&quot;, 
								    dataType: &quot;json&quot;, 
								    data: {method:&quot;getTopicBookMainItemCode&quot;,language:&quot;4&quot;}, 
								    success: function(data) {
										  //alert(&quot;RES=&quot;+data);
						              	  $(&quot;#popTopicBookSelect #L1DropDown&quot;).loadSelect(data);
						              	  popTopicBookSelect.L2_Dropdown();
								    }, 
								    error: function (XMLHttpRequest, textStatus, errorThrown) { 
								    	alert(XMLHttpRequest.responseText);
								    } 
							});
							//bind
					        $(&quot; , &quot;'&quot; , &quot;#popTopicBookSelect #L1dropdown&quot; , &quot;'&quot; , &quot;).change(function(){
						            popTopicBookSelect.L2_Dropdown();
					        });
			                $(&quot;#popTopicBookSelect input.button&quot;).val($(&quot; , &quot;'&quot; , &quot;#popTopicBookSelect #save&quot; , &quot;'&quot; , &quot;).text());
			              	$(&quot;#popTopicBookSelect&quot;).show();
					  })(jQuery);
              },
		      L2_Dropdown : function() {//LEVEL 2----------------------------------------------------------------------------------
					(function($){
				        var L1_value    = $(&quot; , &quot;'&quot; , &quot;#popTopicBookSelect #L1dropdown&quot; , &quot;'&quot; , &quot;).val();
				        var dropdownSet = $(&quot; , &quot;'&quot; , &quot;#popTopicBookSelect #L2dropdown&quot; , &quot;'&quot; , &quot;);
				        if (L1_value.length == 0) {
					          dropdownSet.attr(&quot;disabled&quot;,true);
					          dropdownSet.emptySelect();
				        }else {
					          dropdownSet.attr(&quot;disabled&quot;,false);
						      var contextPath=trim($(&quot; , &quot;'&quot; , &quot;#popTopicBookSelect #contextPath&quot; , &quot;'&quot; , &quot;).text());//取contextPath
					          $.ajax({ 
									type: &quot;POST&quot;, 
									url: contextPath+&quot;/kidMain&quot;, 
									dataType: &quot;json&quot;, 
									data: {method:&quot;getTopicBookSubItemCode&quot;,main_id:trim(L1_value),language:&quot;4&quot;}, 
								    success: function(data) {
						            	dropdownSet.loadSelect(data);
								    }, 
								    error: function (XMLHttpRequest, textStatus, errorThrown) { 
								    	//alert(XMLHttpRequest.responseText);
								    	alert(&quot;no data&quot;);
								    } 
							  });
				        }
					})(jQuery);
		      },
		      cancel : function() {//CANCEL----------------------------------------------------------------------------
					(function($){
							$(&quot;#popTopicBookSelect&quot;).hide();
					})(jQuery);
			  },
		      checkBoxClear : function() {//checkBox Clear----------------------------------------------------------------------------
					(function($){
						$(&quot;#JQ_ItemCRS input.check&quot;).each(function(){
								$(this).attr(&quot;checked&quot;,false);
						});
						$(&quot;#rootCheck&quot;).attr(&quot;checked&quot;,false);
					})(jQuery);
			  },
		      confirm : function() {//run--------------------------------------------------------------------------
					(function($){
							$(&quot;#popTopicBookSelect&quot;).hide();
							var contextPath=trim($(&quot; , &quot;'&quot; , &quot;#popTopicBookSelect #contextPath&quot; , &quot;'&quot; , &quot;).text());//取contextPath
						    var i=0;
						    var flag=&quot;n&quot;;
						    var selectItems=&quot;&quot;;
							$(&quot;#JQ_ItemCRS input.check&quot;).each(function(){
									//alert(i+&quot; - &quot;+$(this).attr(&quot;checked&quot;));
									var select=$(this).attr(&quot;checked&quot;);
									if(select==true){
										//alert(i+&quot; - &quot;+$(this).parent().parent().find(&quot;td[className=itemId]&quot;).text());
										var itemId=trim($(this).parent().parent().find(&quot;td[className=itemId]&quot;).text());
										if(flag==&quot;y&quot;) selectItems=selectItems+&quot;,&quot;;
										selectItems=selectItems+itemId;
										flag=&quot;y&quot;;
									}
									i++;
							});
							//alert(&quot;good book REQ=&quot;+selectItems);
					        var mainId =$(&quot;#popTopicBookSelect #L1dropdown&quot;).find(&quot; , &quot;'&quot; , &quot;:selected&quot; , &quot;'&quot; , &quot;).val();
					        var subId  =$(&quot;#popTopicBookSelect #L2dropdown&quot;).find(&quot; , &quot;'&quot; , &quot;:selected&quot; , &quot;'&quot; , &quot;).val();
					        //alert(mainId+&quot;/&quot;+subId);
							popTopicBookSelect.checkBoxClear();
					        $.ajax({ 
								    type: &quot;POST&quot;, 
								    url: contextPath+&quot;/kidMain&quot;, 
								    dataType: &quot;text&quot;, 
								    data: {method:&quot;setTopicBooks&quot;,selectItems:selectItems,mainId:mainId,subId:subId}, 
								    success: function(html) {
											//alert(&quot;RES=&quot;+html);
											var success=$(&quot; , &quot;'&quot; , &quot;#popTopicBookSelect #success&quot; , &quot;'&quot; , &quot;).text();
											alert(success);
								    }, 
								    error: function (XMLHttpRequest, textStatus, errorThrown) { 
								    	alert(XMLHttpRequest.responseText);
								    } 
							});
					})(jQuery);
			  }
	}	  


&lt;!--
	#popTopicBookSelect {
		position: absolute;
		left: 250px;
		top: 200px;
		z-index: 1;
	}
	#popTopicBookSelect .pop_foreground {
	z-index:3;
	left: 21px;
	background-color: #FFFFFF;
	position: absolute;
	width: 342px;
	border-top-width: 1px;
	border-right-width: 1px;
	border-bottom-width: 1px;
	border-left-width: 1px;
	border-top-style: solid;
	border-right-style: solid;
	border-bottom-style: solid;
	border-left-style: solid;
	border-top-color: #84A9D8;
	border-right-color: #84A9D8;
	border-bottom-color: #84A9D8;
	border-left-color: #84A9D8;
	}
	#popTopicBookSelect .pop_foreground .pop_cancel {
		position: absolute;
		top: 6px;
		right: 10px;
	}
	#popTopicBookSelect .pop_foreground .pop_cancel a {
		color: #FF0000;
		text-decoration: none;
	}
	#popTopicBookSelect .pop_foreground  .pop_header {
	color: #FFFFFF;
	background-color: #84A9D8;
	margin-top: 0px;
	margin-bottom: 0px;
	}
	#popTopicBookSelect .pop_foreground  .pop_header p {
		margin-top: 0px;
		margin-bottom: 0px;
		font-size: 12pt;
		line-height: 2em;
		font-weight: bold;
	}
	#popTopicBookSelect .pop_foreground .pop_content {
		width: 100%;
		margin-top: 0px;
		margin-bottom: 0px;
		border: 1px solid #666666;
		background: #FFFFFF;
	}
	#popTopicBookSelect .pop_foreground .pop_content a {
		text-decoration: none;
		font-size: 12pt;
		line-height: 1em;
		color: #000000;
		display: block;
	}
	#popTopicBookSelect .pop_foreground .pop_content a:hover {
		text-decoration: none;
		font-size: 12pt;
		line-height: 1em;
		color: #FFFFFF;
		background: #a69f00;
	}
	#popTopicBookSelect .pop_foreground  .pop_content p {
		margin-top: 0px;
		margin-bottom: 0px;
		font-size: 12pt;
		line-height: 1em;
	}
	#popTopicBookSelect .pop_foreground .pop_footer {
		background: #FFA100;
		text-align: center;
		font-size: 12pt;
		line-height: 1em;
		color: #993300;
	}

	#popTopicBookSelect #detailFormContainer {
	width: 322px;
	padding-top: 2px;
	padding-right: 10px;
	padding-bottom: 12px;
	padding-left: 10px;
	}
	#popTopicBookSelect #detailFormContainer br {
			clear: both;
	}
	#popTopicBookSelect #detailFormContainer #cascadingDropdowns div {
		  float: left;
		  margin-right: 10px;
	}
	#popTopicBookSelect #detailFormContainer label {
			float: left;
			margin-right: 10px;
			color: #FFFFFF;
			font: 24px &quot;標楷體&quot;;
	}
#popTopicBookSelect #confirm {
	text-align: right;
	margin-left: 225px;
	margin-top: 20px;
}
#popTopicBookSelect #confirm .button {
	background-color: #84A9D8;
	margin: 0px;
	border: 1px outset #CCC;
}
-->



		/inspireapp/
		
		
	    
	    	
	        
	          	Good Book Setup
	        
	        
      			
                    
                    
                          
	                          
	                            primary
	                            
	                          
	                          
	                            secondary
	                            
	                          
	                          
 		    						
							  
                          
                    
                    
                
            
	    

	
	

	 

			


		

		 

	

	

 
  
    Go To Page
  
  
 
 
  
 


 
		
	 






	
	var tm_timerID = window.setInterval(&quot;refreshTaskManager()&quot;, 10000);
	var init = 0;
	function refreshTaskManager() {
		var status = &quot; , &quot;'&quot; , &quot;S1&quot; , &quot;'&quot; , &quot;;
		var execute = 0;
		if (document.getElementById(&quot; , &quot;'&quot; , &quot;tmArea&quot; , &quot;'&quot; , &quot;) != null) {
			execute = 1;
			//status = document.getElementById(&quot; , &quot;'&quot; , &quot;taskStatus&quot; , &quot;'&quot; , &quot;).value;
		}
		if (execute == 1) {
			if ((status == &quot; , &quot;'&quot; , &quot;S1&quot; , &quot;'&quot; , &quot;) || (init == 0)) {
				document.getElementById(&quot; , &quot;'&quot; , &quot;taskManagerRefreshLink&quot; , &quot;'&quot; , &quot;).onclick();
			} else {
				clearInterval(tm_timerID);
			}
			init++;
		}
	}
	


 
  
  
     
  
 
 
  
 



	
	


 
  
    更改館藏狀態
  
  
 
 
  
 




		
	var tm_timerID = window.setInterval(&quot;refreshTaskManager()&quot;, 10000);
	var init = 0;
	function refreshTaskManager() {
		var status = &quot; , &quot;'&quot; , &quot;S1&quot; , &quot;'&quot; , &quot;;
		var execute = 0;
		if (document.getElementById(&quot; , &quot;'&quot; , &quot;tmArea&quot; , &quot;'&quot; , &quot;) != null) {
			execute = 1;
			//status = document.getElementById(&quot; , &quot;'&quot; , &quot;taskStatus&quot; , &quot;'&quot; , &quot;).value;
		}
		if (execute == 1) {
			if ((status == &quot; , &quot;'&quot; , &quot;S1&quot; , &quot;'&quot; , &quot;) || (init == 0)) {
				document.getElementById(&quot; , &quot;'&quot; , &quot;taskManagerRefreshLink&quot; , &quot;'&quot; , &quot;).onclick();
			} else {
				clearInterval(tm_timerID);
			}
			init++;
		}
	}
	


 
  
  
     
  
 
 
  
 




		
	

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
var field1 = new Ajax.Autocompleter(&quot;field1&quot;, &quot;field1choices&quot;, &quot;/inspireapp/internaltranzit/ManageInBatch,searchComponent.field1.sdirect?sp=Sfield1&amp;sp=S2&amp;sp=Sstarts+with&amp;sp=1&amp;updateParts=field1&quot;, {&quot;method&quot;:&quot;get&quot;,&quot;frequency&quot;:0.2,&quot;minChars&quot;:1,&quot;onFailure&quot;:tapestry.error,&quot;encoding&quot;:&quot;UTF-8&quot;});
var field2 = new Ajax.Autocompleter(&quot;field2&quot;, &quot;field2choices&quot;, &quot;/inspireapp/internaltranzit/ManageInBatch,searchComponent.field2.sdirect?sp=Sfield2&amp;sp=S30&amp;sp=Sstarts+with&amp;sp=2&amp;updateParts=field2&quot;, {&quot;method&quot;:&quot;get&quot;,&quot;frequency&quot;:0.2,&quot;minChars&quot;:1,&quot;onFailure&quot;:tapestry.error,&quot;encoding&quot;:&quot;UTF-8&quot;});
var field3 = new Ajax.Autocompleter(&quot;field3&quot;, &quot;field3choices&quot;, &quot;/inspireapp/internaltranzit/ManageInBatch,searchComponent.field3.sdirect?sp=Sfield3&amp;sp=S5&amp;sp=Sstarts+with&amp;sp=3&amp;updateParts=field3&quot;, {&quot;method&quot;:&quot;get&quot;,&quot;frequency&quot;:0.2,&quot;minChars&quot;:1,&quot;onFailure&quot;:tapestry.error,&quot;encoding&quot;:&quot;UTF-8&quot;});
var field4 = new Ajax.Autocompleter(&quot;field4&quot;, &quot;field4choices&quot;, &quot;/inspireapp/internaltranzit/ManageInBatch,searchComponent.field4.sdirect?sp=Sfield4&amp;sp=S7&amp;sp=Sstarts+with&amp;sp=4&amp;updateParts=field4&quot;, {&quot;method&quot;:&quot;get&quot;,&quot;frequency&quot;:0.2,&quot;minChars&quot;:1,&quot;onFailure&quot;:tapestry.error,&quot;encoding&quot;:&quot;UTF-8&quot;});
tapestry.cleanConnect(&quot;formSubmitSearch&quot;, &quot;onclick&quot;, &quot;event1167465096&quot;);
        tapestry.event1167465096=function(e){
            tapestry.event.stopEvent(e);
            
                
                    tapestry.form.submit(&quot;searchForm&quot;, &quot;formSubmitSearch&quot;, {&quot;async&quot;:true,&quot;json&quot;:false,&quot;url&quot;:&quot;/inspireapp/internaltranzit/ManageInBatch,searchComponent.searchForm.sdirect?updateParts=results&amp;updateParts=nrResults&amp;updateParts=autoRefreshZone&amp;updateParts=zSources&amp;updateParts=localSources&amp;updateParts=showOrderBy&amp;updateParts=noticeMessage&quot;});
                
                
            
            
        };
        tapestry.connect(&quot;formSubmitSearch&quot;, &quot;onclick&quot;, &quot;event1167465096&quot;);
tapestry.cleanConnect(&quot;browse&quot;, &quot;onclick&quot;, &quot;event1984482014&quot;);
        tapestry.event1984482014=function(e){
            tapestry.event.stopEvent(e);
            
                
                    tapestry.form.submit(&quot;searchForm&quot;, &quot;browse&quot;, {&quot;async&quot;:true,&quot;json&quot;:false,&quot;url&quot;:&quot;/inspireapp/internaltranzit/ManageInBatch,searchComponent.searchForm.sdirect?updateParts=results&amp;updateParts=nrResults&amp;updateParts=autoRefreshZone&amp;updateParts=inputField5&quot;});
                
                
            
            
        };
        tapestry.connect(&quot;browse&quot;, &quot;onclick&quot;, &quot;event1984482014&quot;);
tapestry.cleanConnect(&quot;Submit&quot;, &quot;onclick&quot;, &quot;event178957379&quot;);
        tapestry.event178957379=function(e){
            tapestry.event.stopEvent(e);
            
                
                    tapestry.form.submit(&quot;searchForm&quot;, &quot;Submit&quot;, {&quot;async&quot;:true,&quot;json&quot;:false,&quot;url&quot;:&quot;/inspireapp/internaltranzit/ManageInBatch,searchComponent.searchForm.sdirect?updateParts=results&amp;updateParts=nrResults&amp;updateParts=autoRefreshZone&amp;updateParts=inputField5&quot;});
                
                
            
            
        };
        tapestry.connect(&quot;Submit&quot;, &quot;onclick&quot;, &quot;event178957379&quot;);

tapestry.cleanConnect(&quot;sCriteria1&quot;, &quot;onchange&quot;, &quot;formEvent1172116752&quot;);
                tapestry.formEvent1172116752=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;internaltranzit/ManageInBatch/searchComponent.sCriteria1&quot;, bcomponentid:&quot;sCriteria1&quot;};
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
                tapestry.connect(&quot;sCriteria1&quot;, &quot;onchange&quot;, &quot;formEvent1172116752&quot;);
tapestry.cleanConnect(&quot;comparator1&quot;, &quot;onchange&quot;, &quot;formEvent308899038&quot;);
                tapestry.formEvent308899038=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;internaltranzit/ManageInBatch/searchComponent.comparator1&quot;, bcomponentid:&quot;comparator1&quot;};
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
                tapestry.connect(&quot;comparator1&quot;, &quot;onchange&quot;, &quot;formEvent308899038&quot;);
tapestry.cleanConnect(&quot;sCriteria2&quot;, &quot;onchange&quot;, &quot;formEvent1335983496&quot;);
                tapestry.formEvent1335983496=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;internaltranzit/ManageInBatch/searchComponent.sCriteria2&quot;, bcomponentid:&quot;sCriteria2&quot;};
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
                tapestry.connect(&quot;sCriteria2&quot;, &quot;onchange&quot;, &quot;formEvent1335983496&quot;);
tapestry.cleanConnect(&quot;comparator2&quot;, &quot;onchange&quot;, &quot;formEvent1479350283&quot;);
                tapestry.formEvent1479350283=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;internaltranzit/ManageInBatch/searchComponent.comparator2&quot;, bcomponentid:&quot;comparator2&quot;};
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
                tapestry.connect(&quot;comparator2&quot;, &quot;onchange&quot;, &quot;formEvent1479350283&quot;);
tapestry.cleanConnect(&quot;sCriteria3&quot;, &quot;onchange&quot;, &quot;formEvent886327579&quot;);
                tapestry.formEvent886327579=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;internaltranzit/ManageInBatch/searchComponent.sCriteria3&quot;, bcomponentid:&quot;sCriteria3&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;sCriteria3&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;sCriteria3&quot;, &quot;onchange&quot;, &quot;formEvent886327579&quot;);
tapestry.cleanConnect(&quot;comparator3&quot;, &quot;onchange&quot;, &quot;formEvent1930191548&quot;);
                tapestry.formEvent1930191548=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;internaltranzit/ManageInBatch/searchComponent.comparator3&quot;, bcomponentid:&quot;comparator3&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;comparator3&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;comparator3&quot;, &quot;onchange&quot;, &quot;formEvent1930191548&quot;);
tapestry.cleanConnect(&quot;sCriteria4&quot;, &quot;onchange&quot;, &quot;formEvent588250658&quot;);
                tapestry.formEvent588250658=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;internaltranzit/ManageInBatch/searchComponent.sCriteria4&quot;, bcomponentid:&quot;sCriteria4&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;sCriteria4&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;sCriteria4&quot;, &quot;onchange&quot;, &quot;formEvent588250658&quot;);
tapestry.cleanConnect(&quot;comparator4&quot;, &quot;onchange&quot;, &quot;formEvent851833462&quot;);
                tapestry.formEvent851833462=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;internaltranzit/ManageInBatch/searchComponent.comparator4&quot;, bcomponentid:&quot;comparator4&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;comparator4&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;comparator4&quot;, &quot;onchange&quot;, &quot;formEvent851833462&quot;);
tapestry.cleanConnect(&quot;browseCriteria&quot;, &quot;onchange&quot;, &quot;formEvent1302783030&quot;);
                tapestry.formEvent1302783030=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;internaltranzit/ManageInBatch/searchComponent.browseCriteria&quot;, bcomponentid:&quot;browseCriteria&quot;};
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
                tapestry.connect(&quot;browseCriteria&quot;, &quot;onchange&quot;, &quot;formEvent1302783030&quot;);
closeDialogComponent(&quot; , &quot;'&quot; , &quot;TinreadDialog&quot; , &quot;'&quot; , &quot;);

closeDialogComponent(&quot; , &quot;'&quot; , &quot;GlobalChangeStatus&quot; , &quot;'&quot; , &quot;);
try {
  attachFocus(&quot; , &quot;'&quot; , &quot;field1&quot; , &quot;'&quot; , &quot;);
 }
 catch(e) {}
closeDialogComponent(&quot; , &quot;'&quot; , &quot;IGCD&quot; , &quot;'&quot; , &quot;);
closeDialogComponent(&quot; , &quot;'&quot; , &quot;GlobalChangeStatus_1&quot; , &quot;'&quot; , &quot;);
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



            
                
                    確定
                    取消
                
                
                    
                        
                            CMUL - 神資圖書館
                            
                                
                            
                        
                    
                    
                    
                        
                            2 - 2
                            
                                
                            
                        
                    
                    
                
                    
                        
                            123 - 123
                            
                                
                            
                        
                    
                    
                
                    
                        
                            20230417 - 20230417
                            
                                
                            
                        
                    
                    
                
                    
                        
                            20230418 - 20230418
                            
                                
                            
                        
                    
                    
                
                    
                        
                            AH - 安南醫院
                            
                                
                            
                        
                    
                    
                    
                        
                            AHGL - 安南圖書區
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            av - av
                            
                                
                            
                        
                    
                    
                
                    
                        
                            B007 - B007
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BCSB4 - BCSB4
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BX - 取書櫃1
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BY - 取書櫃2
                            
                                
                            
                        
                    
                    
                
                    
                        
                            CB - 北港分館
                            
                                
                            
                        
                    
                    
                    
                        
                            BAVN - 北港分館視聽區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BCIR - 北港分館流通櫃檯
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BCRA - 北港分館指參(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BPAV - 北港分館視聽區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BPCL - 北港分館書庫
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            cbook - cbook
                            
                                
                            
                        
                    
                    
                
                    
                        
                            circd - circd
                            
                                
                            
                        
                    
                    
                
                    
                        
                            clp - clp
                            
                                
                            
                        
                    
                    
                
                    
                        
                            CM - 北港附設醫院
                            
                                
                            
                        
                    
                    
                    
                        
                            BMHL - 北港附設醫院圖書室
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            CMUL - 中國醫藥大學圖書館 - CMUL - 中國醫藥大學圖書館
                            
                                
                            
                        
                    
                    
                
                    
                        
                            CU - 台中總館
                            
                                
                            
                        
                    
                    
                    
                        
                            CUAV - 台中總館視聽區(獨立專區)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MAVN - 台中總館視聽區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MAVR - 台中總館視聽區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCAT - 台中總館技服組
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCBS - 台中總館密閉書庫
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCIR - 台中總館流通櫃檯
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCRA - 台中總館教師指定參考書(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCSS - 台中總館B3裝訂期刊區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MDIA - 台中總館博碩士論文區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MEAS - 台中總館探索史懷哲之路專書區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MEXM - 台中總館國考書區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MFGA - 台中總館本校教職優良教材區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MFPA - 台中總館本校教師升等資料區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MFSA - 台中總館本校教職論著
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MHME - 台中總館人文專書區-醫學倫理(限館內閱覽)台中總館人文專書區-醫學倫理(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MHMH - 台中總館人文專書區-醫療史(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MHML - 台中總館人文專書區-醫學法律(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MHQC - 台中總館人文專書區-醫療品質(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MNBR - 台中總館新書展示區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MPAA - 台中總館績效暨獲獎區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MPBL - 台中總館PBL專書區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MREF - 台中總館參考室
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MRSS - 台中總館閱覽組
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MS-C - 台中總館期刊複本櫃
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSER - 台中總館期刊區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSPA - 台中總館研究計劃專書
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSPB - 中醫醫史文獻室(限所內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSPC - 台中總館特藏室
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSTK - 台中總館書庫
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MYBK - 台中總館參考壁櫃
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 7 - new item 7
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ONLN - 台中總館線上資料
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            e-resources - 電子資源
                            
                                
                            
                        
                    
                    
                
                    
                        
                            EB-P - EB-P
                            
                                
                            
                        
                    
                    
                
                    
                        
                            elect - elect
                            
                                
                            
                        
                    
                    
                
                    
                        
                            H-EQ - H-EQ
                            
                                
                            
                        
                    
                    
                
                    
                        
                            H-MR - H-MR
                            
                                
                            
                        
                    
                    
                
                    
                        
                            L - L
                            
                                
                            
                        
                    
                    
                
                    
                        
                            L40 - L40
                            
                                
                            
                        
                    
                    
                
                    
                        
                            LB 圖書總館 - LB 圖書總館
                            
                                
                            
                        
                    
                    
                
                    
                        
                            LB-S - LB-S
                            
                                
                            
                        
                    
                    
                
                    
                        
                            LE - 語文教學中心
                            
                                
                            
                        
                    
                    
                    
                        
                            LEGL - 語文教學中心圖書室
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            LIB - LIB
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 1 - new item 1
                            
                                
                            
                        
                    
                    
                    
                        
                            new item 3 - new item 3
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            new item 10 - new item 10
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 12 - new item 12
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 13 - new item 13
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 14 - new item 14
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 16 - new item 16
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 2 - new item 2
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 20 - new item 20
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 4 - new item 4
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 5 - new item 5
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 6 - 英才校區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 8 - new item 8
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 9 - new item 9
                            
                                
                            
                        
                    
                    
                
                    
                        
                            NPTU - NPTU
                            
                                
                            
                        
                    
                    
                
                    
                        
                            OUK - OUK
                            
                                
                            
                        
                    
                    
                
                    
                        
                            PT - 培德醫院
                            
                                
                            
                        
                    
                    
                    
                        
                            new item 11 - new item 11
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 17 - new item 17
                            
                                
                            
                        
                    
                    
                
                    
                        
                            PTGL - 培德醫院圖書區
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            ptext - ptext
                            
                                
                            
                        
                    
                    
                
                    
                        
                            SB3 - SB3
                            
                                
                            
                        
                    
                    
                
                    
                        
                            T-P - T-P
                            
                                
                            
                        
                    
                    
                
                    
                        
                            TBBK - TBBK
                            
                                
                            
                        
                    
                    
                
                    
                        
                            TH - 台北分院
                            
                                
                            
                        
                    
                    
                    
                        
                            THGL - 台北分院圖書區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            THPA - 台北分院期刊區
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            W-P - W-P
                            
                                
                            
                        
                    
                    
                
                    
                        
                            YH - 豐原分院
                            
                                
                            
                        
                    
                    
                    
                        
                            new item 18 - new item 18
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 19 - new item 19
                            
                                
                            
                        
                    
                    
                
                    
                        
                            YHGL - 豐原分院圖書區
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            z3llc - z3llc
                            
                                
                            
                        
                    
                    
                
                    
                        
                            z6bkf - z6bkf
                            
                                
                            
                        
                    
                    
                
                    
                        
                            zd1a2 - zd1a2
                            
                                
                            
                        
                    
                    
                
                    
                        
                            zd1e - zd1e
                            
                                
                            
                        
                    
                    
                
                    
                        
                            zdlf - zdlf
                            
                                
                            
                        
                    
                    
                
                    
                        
                            五樓漫畫書專區 - 五樓漫畫書專區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            實體館藏 - 實體館藏
                            
                                
                            
                        
                    
                    
                
                    
                        
                            綜合書庫 - 綜合書庫
                            
                                
                            
                        
                    
                    
                
                    
                        
                            艾迪訊圖書館 - 艾迪訊圖書館
                            
                                
                            
                        
                    
                    
                
                    
                        
                            附中出版物專區 - 附中出版物專區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            龍華科技大學圖書館 - 龍華科技大學圖書館
                            
                                
                            
                        
                    
                    
                
                
            
                
                    確定
                    取消
                
                
                    
                        
                            B可借圖書
                            
                                
                            
                        
                    
                    
                
                    
                        
                            B電子資源
                            
                                
                            
                        
                    
                    
                
                    
                        
                            eee
                            
                                
                            
                        
                    
                    
                
                    
                        
                            M可借行動設備
                            
                                
                            
                        
                    
                    
                
                    
                        
                            P可借期刊
                            
                                
                            
                        
                    
                    
                
                    
                        
                            V可借視聽
                            
                                
                            
                        
                    
                    
                
                    
                        
                            www
                            
                                
                            
                        
                    
                    
                
                    
                        
                            不流通
                            
                                
                            
                        
                    
                    
                
                    
                        
                            書箱借閱30天
                            
                                
                            
                        
                    
                    
                
            
                
                    確定
                    取消
                
                
                    
                        
                            BD - 藍光光碟
                            
                                
                            
                        
                    
                    
                
                    
                        
                            CA - 靜畫資料
                            
                                
                            
                        
                    
                    
                
                    
                        
                            DB - 資料庫
                            
                                
                            
                        
                    
                    
                
                    
                        
                            DF - 磁片
                            
                                
                            
                        
                    
                    
                
                    
                        
                            DO - 電子書
                            
                                
                            
                        
                    
                    
                
                    
                        
                            EA - 立體模型
                            
                                
                            
                        
                    
                    
                
                    
                        
                            EB - 線上電子書
                            
                                
                            
                        
                    
                    
                
                    
                        
                            EJ - 線上電子期刊
                            
                                
                            
                        
                    
                    
                
                    
                        
                            EP - 電子期刊光碟
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ERROR - 有問題特藏
                            
                                
                            
                        
                    
                    
                
                    
                        
                            FA - 磁帶
                            
                                
                            
                        
                    
                    
                
                    
                        
                            KT - 多媒體組件
                            
                                
                            
                        
                    
                    
                
                    
                        
                            LA - 地圖
                            
                                
                            
                        
                    
                    
                
                    
                        
                            LD - 影碟
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MP - MP3
                            
                                
                            
                        
                    
                    
                
                    
                        
                            NH - 微縮單片
                            
                                
                            
                        
                    
                    
                
                    
                        
                            NR - 微縮捲片
                            
                                
                            
                        
                    
                    
                
                    
                        
                            QA - 地球儀
                            
                                
                            
                        
                    
                    
                
                    
                        
                            R - 參考書
                            
                                
                            
                        
                    
                    
                
                    
                        
                            SL - 幻燈片
                            
                                
                            
                        
                    
                    
                
                    
                        
                            VC - 錄影帶
                            
                                
                            
                        
                    
                    
                
                    
                        
                            VD - VCD
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BOX - 書箱
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ERM_DB - 電子資料庫(ERM)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ERM_WS - 網路資源(ERM)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ERM_EB - 電子書(ERM)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ERM_EJ - 電子期刊(ERM)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            XL - X-ray
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BOOK - 圖書
                            
                                
                            
                        
                    
                    
                
                    
                        
                            AC - 錄音帶
                            
                                
                            
                        
                    
                    
                
                    
                        
                            APP - 附件
                            
                                
                            
                        
                    
                    
                
                    
                        
                            P - 現期期刊
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ac - ac
                            
                                
                            
                        
                    
                    
                
                    
                        
                            DD - 影像光碟(DVD)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MD - 行動設備
                            
                                
                            
                        
                    
                    
                
                    
                        
                            S - 裝訂期刊
                            
                                
                            
                        
                    
                    
                
                    
                        
                            AD - 唱片
                            
                                
                            
                        
                    
                    
                
                    
                        
                            CD - 光碟
                            
                                
                            
                        
                    
                    
                
                    
                        
                            booklet - 小冊子
                            
                                
                            
                        
                    
                    
                
                    
                        
                            KKtest - KK
                            
                                
                            
                        
                    
                    
                
                    
                        
                            YYtest2 - YY2
                            
                                
                            
                        
                    
                    
                
                    
                        
                            YYtest4 - YYYtest
                            
                                
                            
                        
                    
                    
                
                    
                        
                            0425 - 0425
                            
                                
                            
                        
                    
                    
                
                    
                        
                            TEST - TEST
                            
                                
                            
                        
                    
                    
                
                    
                        
                            TEST0425 - TEST0425
                            
                                
                            
                        
                    
                    
                id(&quot;field2&quot;)&quot;) or . = concat(&quot;

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
				
				最近登入:2024-03-12 15:47:21 ,成功
				
				
					
					
                       0 筆新訊息
                    
                
				
				繁體中文English繁體中文
				
English
繁體中文
						
						
				
				
					
					登 出
				
			
		
		
		
			
			
				神資圖書館(15trunk機)
			
			
		
	
	
	
	
	
		
		
		
			
			
			 查詢  編目  流通  採購   期刊控制   場地設備  說明  我的帳戶  學科服務  管理  統計  清單  Portal  臉辨報表  ERM 
			簡單查詢進階查詢FRBR查詢最近的查詢已儲存的記錄FRBR作品書目書目記錄維護註記刪除之書目記錄書目記錄全域修改權威記錄權威記錄維護註記刪除之權威記錄權威記錄全域修改條碼號館藏資料維護註記刪除之館藏資料維護出版者出版者資料維護內部移送作業書單維護FRBR作品辦證讀者記錄維護檢查讀者註記刪除之讀者記錄讀者證卡記錄維護離職離校借閱檯讀者流通檯還書箱讀者查詢交易紀錄違規讀者查詢讀者個人違規記錄指定參考書交易歷史查詢一般搜尋交易館藏預約順位調整題名預約順位調整批次修改到期日內部移送作業盤點盤點維護跨館預約/調閱跨館預約/調閱需求移轉-寄送項目移轉-收到項目盤點比對批次報廢選擇報廢執行報廢已報廢歷史查詢書箱作業條碼號空號清單(條碼機印製)訂購檢查薦購請購訂購明細訂單登收用卷期登收程序發票款項維護預算經費供應商標案內部移送作業催缺候選催缺項目催缺記錄期刊主檔訂購檢查薦購請購核可訂購明細訂單登收用卷期登收程序發票款項維護預算經費供應商裝訂裝訂主檔維護擬裝設定送裝確認裝訂登收標案內部移送作業催缺候選催缺項目催缺記錄流通記錄預約記錄借用記錄維護分類維護單項維護參數設定流通政策維護預約審核場地設備黑名單書籤訊息排程工作日誌學科館藏館藏資料類型設定學科館藏維護學科預約教師資料維護讀者評價選單學科預約維護學科預約查詢教師評語學科報表學科館藏冊數統計教師學科服務時數與使用人次統計學科服務讀者清單學科報表學科館藏冊數統計教師學科服務時數與使用人次統計學科服務讀者清單一般設定類型/使用者(館員)片語設定系統設定貨幣匯率館藏地範本設定--依館員帳號翻譯轉換 選取清單格式屬性條碼號規範顯示設定圖書館系統屬性推薦範本設定館員新增申請QR Code 設定編目書目類型設定MARC規範格式設定顯示設定停用字SDI維護同義字條碼號原則對應流通流通通知單範本流通通知單參數設定流通通知單發送紀錄日常作業排程一般每周開閉館設定閉館日期維護交易紀錄類型交易方式可適用讀者身份交易方式可適用館藏流通類別交易方式可適用使用類別交易紀錄-館藏狀態流通政策讀者證卡原則單位所系夾書單欄位設定讀者閱讀興趣發送紀錄期刊週期傳閱清單維護 轉入Z39屬性設定更新書封上傳書封索引編修索引重建功能維護公佈欄維護意見回覆維護主題維護特別書籍維護心得分享東區互惠審核東區互惠匯出系統代碼維護貨幣代碼作者號維護科特號維護OPACSDI維護讀者評論管理Help維護熱門關鍵字過濾館員Log記錄館員登入/出記錄館員資料變更記錄讀者資料變更記錄權限設定MARC權限設定功能權限設定角色設定系統狀態排程管理排程監控者排程日誌查詢統計檢索點/檢索語統計報表不同時段使用次數統計書目下載次數統計電子資源統計電子資源-館藏點閱次數統計電子資源-讀者點閱次數統計電子資源-單位系所點閱次數統計編目統計館藏量統計報表類號區間統計報表圖書增減統計報表編目/館別/大類交叉分析報表大類/冊數統計表大類/特藏交叉統計表特藏/冊數統計表新書特藏大類交叉統計圖書增減表／月份／大類大類語言種數冊數統計大類/條碼號類別冊數統計圖書冊數分類統計單圖書冊數金額分類統計非書資料數量金額統計資料類型範圍館藏統計表編目工作量統計(個人)編目工作量統計(各館)館藏狀態統計報表流通統計讀者借閱排行榜書展借閱清單流通人數統計(學制分頁 符合科系)讀者類型借閱暨歸還流通量統計報表流通人次冊數統計  利用概況統計(日報表)讀者借閱交叉分析各館特藏預約統計報表館內使用統計讀者身份借閱統計讀者個人及單位所系借閱排行榜館藏資料借閱統計單位系所借閱排行榜單位系所借閱統計熱門借閱排行榜熱門預約排行榜讀者辦証暨補發人次統計表借閱交叉分析統計資料類型流通統計館員流通量統計單位(個人)保管圖書(非書)數量明細館際互借代還統計-本館館藏館際互借代還統計-他館館藏指定參考書借閱量統計表各大類熱門借閱排行榜大專院校圖書館讀者身份借閱清單及統計報表館藏流通量統計跨館預約/調閱館藏借閱排行榜罰款收款人數/次數統計跨館預約/調閱使用量統計罰款收款統計特定館藏借閱統計採購統計書商採購統計年度經費採購統計特藏訂購方式金額統計書商到書率書商績效統計期刊統計期刊到刊率統計報表平均收刊日統計報表歷年期刊借閱統計書商到書率書商催缺次數書商累計訂購次數期刊訂費漲幅統計逾期違約金統計期刊經費訂購統計報表場地設備場地設備使用統計表報表模組範本維護編目清單書目標籤報表Tag856檢查報表新版書標維護次分類交叉分析登錄簿條碼產生清單登錄簿清單條碼號空號清單單位科系館藏設定清單館藏狀態預約清單特殊館藏清單館藏狀態清單報表流通清單罰款未收款報表預約待取清單借閱(逾期)清單還書清單新書清單讀者欠款清單借閱逾期清單罰款收款清單預約撤架清單長期借閱圖書清單離線流通清單預約額滿資料清單單位系所借閱逾期清單跨館還書清單離職離校清單跨館移送清單預約保留結束清單指定參考書借閱量明細表未外借館藏清單重複讀者比對報表借閱逾期通知單取書櫃上架清單預約圖書清單採購清單年度經費採買書籍運用率訂購資料明細表擬購資料明細表請購單詢價單訂購逾期清單謝函書的催缺報表薦購書刊複本比對報表採購參考清單財產增加清冊財產減少清冊已付款未到清單訂購明細期刊清單期刊合訂本登錄簿交贈期刊清單期刊架位清單裝訂歷史清單薦購書刊複本比對報表期刊登錄簿索贈清單期刊查驗清單已付款未到清單場地設備預約取消清單臉辨用戶使用分析表臉辨用戶與圖書分類關係表期間辦證統計_年報期間辦證統計_月報期間辦證明細期間辦證異動統計_年報期間辦證異動統計_月報辦證異動明細臉辨使用紀錄統計_年報臉辨使用紀錄統計_月報臉辨使用紀錄明細人數統計表人數統計明細表人次統計表人次統計明細表熱門時段統計表讀者進出排行榜期間辦證刪除明細系統參數設定IP區間管理延伸查詢設定系統設定檔資源瀏覽/後分類設定資源到期通知設定可使用身分類別設定 代碼管理所屬資料庫清單共用代碼檔共用代碼類別電子資源管理資源前端顯示設定Ezproxy設定檔維護電子資料庫/網路資源電子期刊/電子書電子資源批次匯入電子資源副檔匯入讀者回報資源連線異常 Meniuri.erm.9000報表點閱資源排行榜依學院別使用統計依單位別使用統計依月分資源使用統計依學院登入統計表依系所登入統計表依身分登入統計表登入清單依身分別使用統計表使用者使用資源清單 收藏資源排行榜偵測電子資源狀態設定偵測資源時間設定偵測電子資源查詢電子資源狀態電子資源狀況報表
				
	


		
			
				編目 > 內部移送作業		
			
 	



	j(function () {
		j(&quot;div[id=&quot; , &quot;'&quot; , &quot;HiddenDiacritics&quot; , &quot;'&quot; , &quot;]&quot;).draggable({
  	      containment: &quot;#box&quot;,
 	       scroll: false
	});
	});
	j(function () {
		j(&quot;div[id=&quot; , &quot;'&quot; , &quot;hiddenDiacritics&quot; , &quot;'&quot; , &quot;]&quot;).draggable({
	        containment: &quot;#box&quot;,
	        scroll: false
	});
	});
 

	function syncWithOrder() {
		document.getElementById(&quot;orderCriteria&quot;).value = document
				.getElementById(&quot;browseCriteria&quot;).value;
	}
	function syncWithBrowse() {
		document.getElementById(&quot;browseCriteria&quot;).value = document
				.getElementById(&quot;orderCriteria&quot;).value;
	}

	var refreshTime = 0;

	function apasa() {
		refreshTime = 2500;
		clickLinkSubmit(&quot;searchForm&quot;, &quot;scriptSubmit&quot;);
	}

	function showingOrderBy(status) {
		document.getElementById(&quot;showOrderBy&quot;).style.display = status;
	}
	j(document).ready(function() {
		j(&quot;#resetbutton&quot;).click(function() {
			//j(&quot;#reseter&quot;).click();
			var h = j(&quot;#reseter&quot;).attr(&quot;href&quot;);
			window.location = h;
		});

	    // JSON 資料結構模擬 start
	    var treeJsonData = JSON.parse(j(&quot; , &quot;'&quot; , &quot;#locData&quot; , &quot;'&quot; , &quot;).text());
	    
	    j(&quot; , &quot;'&quot; , &quot;.place&quot; , &quot;'&quot; , &quot;).treeoptions({
	        data: treeJsonData,
	        openImg: &quot; , &quot;'&quot; , &quot;/inspireapp/images/ico_add.gif&quot; , &quot;'&quot; , &quot;, // img 路徑
	        cleanImg:&quot; , &quot;'&quot; , &quot;/inspireapp/images/clear.gif&quot; , &quot;'&quot; , &quot; // img 路徑
	    });
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
	});

	function createPopEdit(href) {

		popupwindow = window
				.open(
						&quot;&quot;,
						&quot;MeniuCatalogare&quot;,
						&quot;status=no,toolbar=0,scrollbars=yes,menubar=0,titlebar=0,resizable=1,width=1500,height=1500&quot;);
		popupwindow.moveTo(screen.width / 2 - 435, screen.height / 2 - 300);
		popupwindow.focus();

		popupwindow.location = href;

		if (popupwindow == null)
			popupwindow.opener = self;
		return false;

	}

	function createUploadPopEdit(href) {
		popupwindow = window.open(&quot;&quot;, &quot;Upload&quot;,
				&quot;status=no,toolbar=0,scrollbars=yes,menubar=0,titlebar=0,resizable=1,top=&quot;
						+ (screen.height - 150) / 2 + &quot;,left=&quot;
						+ (screen.width - 600) / 2 + &quot;,width=600,height=250&quot;);
		popupwindow.focus();

		popupwindow.location = href;

		if (popupwindow == null)
			popupwindow.opener = self;
		return false;
	}
	
	function runScript(e) {
	    if (e.keyCode == 13) {
	        document.getElementById(&quot;browse&quot;).click();
	        return false;
	    }
	}
	
	function changeModeDialog(msg, href){
		var crs = document.getElementsByClassName(&quot; , &quot;'&quot; , &quot;search_list_c&quot; , &quot;'&quot; , &quot;).length;
		
		if(crs){
			if(confirm(msg)){
				return createUploadPopEdit(href);
			}
			return false;
		}
		else{
			return createUploadPopEdit(href);
		}
	}



	  
		refreshTime = 0;
	
	







	











































		
			
				 
						 查詢
								
				   
						 條碼號輸入模式 
				
				  
				  上傳條碼號
				
			
		

		

		
			
				
					
						查詢條件
					
					
						
書名
出版者/書商/捐贈者
出版地
ISBN/ISSN
系統識別號
條碼號
其他號碼
索書號
分類號
館藏登收日期
館藏新增日期
館藏新增館員姓名
書目建立館員帳號
館藏修改日期
館藏修改館員姓名
館藏修改館員帳號
  
起始以
包含
等於(=)
大於(>=)
小於(&lt;=)
不等於(≠)

						 
								    
								
								
							 健康 ;健康100健康100 ;健康100 /健康101 : 101種天然食物提昇免疫力 /健康1本通 ;健康365 ;健康365 /健康4大基石 : 合理膳食, 適量運動, 戒煙限酒, 心理平衡. /健康6+1
								
								
							 
					
					
						
and
or
not
 
書名
出版者/書商/捐贈者
出版地
ISBN/ISSN
系統識別號
條碼號
其他號碼
索書號
分類號
館藏登收日期
館藏新增日期
館藏新增館員姓名
書目建立館員帳號
館藏修改日期
館藏修改館員姓名
館藏修改館員帳號
  
起始以
包含
等於(=)
大於(>=)
小於(&lt;=)
不等於(≠)

						 
								    
								
								
							 
								
								
							 
					
					
						
and
or
not
 
書名
出版者/書商/捐贈者
出版地
ISBN/ISSN
系統識別號
條碼號
其他號碼
索書號
分類號
館藏登收日期
館藏新增日期
館藏新增館員姓名
書目建立館員帳號
館藏修改日期
館藏修改館員姓名
館藏修改館員帳號
  
起始以
包含
等於(=)
大於(>=)
小於(&lt;=)
不等於(≠)

						  
								    
								
								
							
						
								
								
							
						
					
					
						
and
or
not
 
書名
出版者/書商/捐贈者
出版地
ISBN/ISSN
系統識別號
條碼號
其他號碼
索書號
分類號
館藏登收日期
館藏新增日期
館藏新增館員姓名
書目建立館員帳號
館藏修改日期
館藏修改館員姓名
館藏修改館員帳號
  
起始以
包含
等於(=)
大於(>=)
小於(&lt;=)
不等於(≠)

						  
								    
								
								
							
						
								
								
							
						
					

					
						
							 限制條件
						  
						
						
					
				
				
					
						
					
					
						
						
					
				
			

			
				
			

			
				
					{&quot;data&quot; :[{&quot;name&quot; : &quot;BD - 藍光光碟&quot;},{&quot;name&quot; : &quot;CA - 靜畫資料&quot;},{&quot;name&quot; : &quot;DB - 資料庫&quot;},{&quot;name&quot; : &quot;DF - 磁片&quot;},{&quot;name&quot; : &quot;DO - 電子書&quot;},{&quot;name&quot; : &quot;EA - 立體模型&quot;},{&quot;name&quot; : &quot;EB - 線上電子書&quot;},{&quot;name&quot; : &quot;EJ - 線上電子期刊&quot;},{&quot;name&quot; : &quot;EP - 電子期刊光碟&quot;},{&quot;name&quot; : &quot;ERROR - 有問題特藏&quot;},{&quot;name&quot; : &quot;FA - 磁帶&quot;},{&quot;name&quot; : &quot;KT - 多媒體組件&quot;},{&quot;name&quot; : &quot;LA - 地圖&quot;},{&quot;name&quot; : &quot;LD - 影碟&quot;},{&quot;name&quot; : &quot;MP - MP3&quot;},{&quot;name&quot; : &quot;NH - 微縮單片&quot;},{&quot;name&quot; : &quot;NR - 微縮捲片&quot;},{&quot;name&quot; : &quot;QA - 地球儀&quot;},{&quot;name&quot; : &quot;R - 參考書&quot;},{&quot;name&quot; : &quot;SL - 幻燈片&quot;},{&quot;name&quot; : &quot;VC - 錄影帶&quot;},{&quot;name&quot; : &quot;VD - VCD&quot;},{&quot;name&quot; : &quot;BOX - 書箱&quot;},{&quot;name&quot; : &quot;ERM_DB - 電子資料庫(ERM)&quot;},{&quot;name&quot; : &quot;ERM_WS - 網路資源(ERM)&quot;},{&quot;name&quot; : &quot;ERM_EB - 電子書(ERM)&quot;},{&quot;name&quot; : &quot;ERM_EJ - 電子期刊(ERM)&quot;},{&quot;name&quot; : &quot;XL - X-ray&quot;},{&quot;name&quot; : &quot;BOOK - 圖書&quot;},{&quot;name&quot; : &quot;AC - 錄音帶&quot;},{&quot;name&quot; : &quot;APP - 附件&quot;},{&quot;name&quot; : &quot;P - 現期期刊&quot;},{&quot;name&quot; : &quot;ac - ac&quot;},{&quot;name&quot; : &quot;DD - 影像光碟(DVD)&quot;},{&quot;name&quot; : &quot;MD - 行動設備&quot;},{&quot;name&quot; : &quot;S - 裝訂期刊&quot;},{&quot;name&quot; : &quot;AD - 唱片&quot;},{&quot;name&quot; : &quot;CD - 光碟&quot;},{&quot;name&quot; : &quot;booklet - 小冊子&quot;},{&quot;name&quot; : &quot;KKtest - KK&quot;},{&quot;name&quot; : &quot;YYtest2 - YY2&quot;},{&quot;name&quot; : &quot;YYtest4 - YYYtest&quot;},{&quot;name&quot; : &quot;0425 - 0425&quot;},{&quot;name&quot; : &quot;TEST - TEST&quot;},{&quot;name&quot; : &quot;TEST0425 - TEST0425&quot;}]}{&quot;data&quot; :[{&quot;name&quot; : &quot;B可借圖書&quot;},{&quot;name&quot; : &quot;B電子資源&quot;},{&quot;name&quot; : &quot;eee&quot;},{&quot;name&quot; : &quot;M可借行動設備&quot;},{&quot;name&quot; : &quot;P可借期刊&quot;},{&quot;name&quot; : &quot;V可借視聽&quot;},{&quot;name&quot; : &quot;www&quot;},{&quot;name&quot; : &quot;不流通&quot;},{&quot;name&quot; : &quot;書箱借閱30天&quot;}]}
						
							限制條件
							
						
						
							書目性質:
							
全系統記錄
圖書
期刊
分析款目
合集

						

						
						
						
						
						
							資料類型:
							
					   		
						


						
							館藏地:
							
								
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




								
						

						

						

						

						
						
						
						
							
							館藏狀態:
							
							
							

在架
借出
預約保留
遺失
盤點未到
聲明歸還
採購處理中
移送編目
編目處理中
移送閱覽
流通處理中
此筆盤點
核准跨館移送
等待送回原館藏地
等待轉送其他館藏地
跨館移送中
跨館轉送中
送回原館藏地
報銷中
報銷找回
重新歸架
展示中
已送裝訂
暫時不可提供
待報廢
已報廢
書箱借出
久借未還
尋書未獲
PickListValues.15004108
盤點結果狀態測試
測試選項
CWEN測試

							
						
						
						
							 
							 	館藏流通類別:
							 
							
							
							
						

						
							

							

							

							
							
						
					

				
			

		


		
			
				
					請指定要修改的資訊：
				
					
						館藏狀態
					

在架
盤點未到
採購處理中
移送編目
編目處理中
移送閱覽
流通處理中
此筆盤點
報銷中
報銷找回
重新歸架
展示中
暫時不可提供
久借未還
尋書未獲
PickListValues.15004108
盤點結果狀態測試
測試選項
CWEN測試

				
				
					
						在OPAC顯示
					

不在OPAC上顯示
在OPAC上顯示

					
				
				
					
						館藏地
					
					
							 
	function runScript(e) {
	    if (e.keyCode == 13) {
	        document.getElementById(&quot;browse&quot;).click();
	        return false;
	    }
	}				

 
 
 
  
  
   
  
  
  
  
   
  
  
   
    
   
   
館藏地CMUL - 神資圖書館2 - 2123 - 12320230417 - 2023041720230418 - 20230418AH - 安南醫院AHGL - 安南圖書區av - avB007 - B007BCSB4 - BCSB4BX - 取書櫃1BY - 取書櫃2CB - 北港分館BAVN - 北港分館視聽區(限館內閱覽)BCIR - 北港分館流通櫃檯BCRA - 北港分館指參(限館內閱覽)BPAV - 北港分館視聽區BPCL - 北港分館書庫cbook - cbookcircd - circdclp - clpCM - 北港附設醫院BMHL - 北港附設醫院圖書室CMUL - 中國醫藥大學圖書館 - CMUL - 中國醫藥大學圖書館CU - 台中總館CUAV - 台中總館視聽區(獨立專區)MAVN - 台中總館視聽區(限館內閱覽)MAVR - 台中總館視聽區MCAT - 台中總館技服組MCBS - 台中總館密閉書庫MCIR - 台中總館流通櫃檯MCRA - 台中總館教師指定參考書(限館內閱覽)MCSS - 台中總館B3裝訂期刊區MDIA - 台中總館博碩士論文區MEAS - 台中總館探索史懷哲之路專書區MEXM - 台中總館國考書區(限館內閱覽)MFGA - 台中總館本校教職優良教材區MFPA - 台中總館本校教師升等資料區MFSA - 台中總館本校教職論著MHME - 台中總館人文專書區-醫學倫理(限館內閱覽)台中總館人文專書區-醫學倫理(限館內閱覽)MHMH - 台中總館人文專書區-醫療史(限館內閱覽)MHML - 台中總館人文專書區-醫學法律(限館內閱覽)MHQC - 台中總館人文專書區-醫療品質(限館內閱覽)MNBR - 台中總館新書展示區MPAA - 台中總館績效暨獲獎區(限館內閱覽)MPBL - 台中總館PBL專書區(限館內閱覽)MREF - 台中總館參考室MRSS - 台中總館閱覽組MS-C - 台中總館期刊複本櫃MSER - 台中總館期刊區MSPA - 台中總館研究計劃專書MSPB - 中醫醫史文獻室(限所內閱覽)MSPC - 台中總館特藏室MSTK - 台中總館書庫MYBK - 台中總館參考壁櫃new item 7 - new item 7ONLN - 台中總館線上資料e-resources - 電子資源EB-P - EB-Pelect - electH-EQ - H-EQH-MR - H-MRL - LL40 - L40LB 圖書總館 - LB 圖書總館LB-S - LB-SLE - 語文教學中心LEGL - 語文教學中心圖書室LIB - LIBnew item 1 - new item 1new item 3 - new item 3new item 10 - new item 10new item 12 - new item 12new item 13 - new item 13new item 14 - new item 14new item 16 - new item 16new item 2 - new item 2new item 20 - new item 20new item 4 - new item 4new item 5 - new item 5new item 6 - 英才校區new item 8 - new item 8new item 9 - new item 9NPTU - NPTUOUK - OUKPT - 培德醫院new item 11 - new item 11new item 17 - new item 17PTGL - 培德醫院圖書區ptext - ptextSB3 - SB3T-P - T-PTBBK - TBBKTH - 台北分院THGL - 台北分院圖書區THPA - 台北分院期刊區W-P - W-PYH - 豐原分院new item 18 - new item 18new item 19 - new item 19YHGL - 豐原分院圖書區z3llc - z3llcz6bkf - z6bkfzd1a2 - zd1a2zd1e - zd1ezdlf - zdlf五樓漫畫書專區 - 五樓漫畫書專區實體館藏 - 實體館藏綜合書庫 - 綜合書庫艾迪訊圖書館 - 艾迪訊圖書館附中出版物專區 - 附中出版物專區龍華科技大學圖書館 - 龍華科技大學圖書館
  
  
window.dlocation = new dTree(&quot; , &quot;'&quot; , &quot;window.dlocation&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;messages&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;/inspireapp/images/&quot; , &quot;'&quot; , &quot;); 
window.dlocation.add(0,-1,&quot; , &quot;'&quot; , &quot;館藏地&quot; , &quot;'&quot; , &quot;); 
window.dlocation.add(1,0,&quot;CMUL - \u795E\u8CC7\u5716\u66F8\u9928&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;CMUL - \\u795E\\u8CC7\\u5716\\u66F8\\u9928&quot; , &quot;'&quot; , &quot;, 1, true)&quot;); 
window.dlocation.add(463,1,&quot;2 - 2&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;2 - 2&quot; , &quot;'&quot; , &quot;, 463, true)&quot;); 
window.dlocation.add(583,1,&quot;123 - 123&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;123 - 123&quot; , &quot;'&quot; , &quot;, 583, true)&quot;); 
window.dlocation.add(1183,1,&quot;20230417 - 20230417&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;20230417 - 20230417&quot; , &quot;'&quot; , &quot;, 1183, true)&quot;); 
window.dlocation.add(1203,1,&quot;20230418 - 20230418&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;20230418 - 20230418&quot; , &quot;'&quot; , &quot;, 1203, true)&quot;); 
window.dlocation.add(167,1,&quot;AH - \u5B89\u5357\u91AB\u9662&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;AH - \\u5B89\\u5357\\u91AB\\u9662&quot; , &quot;'&quot; , &quot;, 167, true)&quot;); 
window.dlocation.add(177,167,&quot;AHGL - \u5B89\u5357\u5716\u66F8\u5340&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;AHGL - \\u5B89\\u5357\\u5716\\u66F8\\u5340&quot; , &quot;'&quot; , &quot;, 177, true)&quot;); 
window.dlocation.add(643,1,&quot;av - av&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;av - av&quot; , &quot;'&quot; , &quot;, 643, true)&quot;); 
window.dlocation.add(303,1,&quot;B007 - B007&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;B007 - B007&quot; , &quot;'&quot; , &quot;, 303, true)&quot;); 
window.dlocation.add(883,1,&quot;BCSB4 - BCSB4&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;BCSB4 - BCSB4&quot; , &quot;'&quot; , &quot;, 883, true)&quot;); 
window.dlocation.add(823,1,&quot;BX - \u53D6\u66F8\u6AC31&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;BX - \\u53D6\\u66F8\\u6AC31&quot; , &quot;'&quot; , &quot;, 823, true)&quot;); 
window.dlocation.add(903,1,&quot;BY - \u53D6\u66F8\u6AC32&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;BY - \\u53D6\\u66F8\\u6AC32&quot; , &quot;'&quot; , &quot;, 903, true)&quot;); 
window.dlocation.add(169,1,&quot;CB - \u5317\u6E2F\u5206\u9928&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;CB - \\u5317\\u6E2F\\u5206\\u9928&quot; , &quot;'&quot; , &quot;, 169, true)&quot;); 
window.dlocation.add(179,169,&quot;BAVN - \u5317\u6E2F\u5206\u9928\u8996\u807D\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;BAVN - \\u5317\\u6E2F\\u5206\\u9928\\u8996\\u807D\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 179, true)&quot;); 
window.dlocation.add(180,169,&quot;BCIR - \u5317\u6E2F\u5206\u9928\u6D41\u901A\u6AC3\u6AAF&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;BCIR - \\u5317\\u6E2F\\u5206\\u9928\\u6D41\\u901A\\u6AC3\\u6AAF&quot; , &quot;'&quot; , &quot;, 180, true)&quot;); 
window.dlocation.add(181,169,&quot;BCRA - \u5317\u6E2F\u5206\u9928\u6307\u53C3(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;BCRA - \\u5317\\u6E2F\\u5206\\u9928\\u6307\\u53C3(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 181, true)&quot;); 
window.dlocation.add(182,169,&quot;BPAV - \u5317\u6E2F\u5206\u9928\u8996\u807D\u5340&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;BPAV - \\u5317\\u6E2F\\u5206\\u9928\\u8996\\u807D\\u5340&quot; , &quot;'&quot; , &quot;, 182, true)&quot;); 
window.dlocation.add(183,169,&quot;BPCL - \u5317\u6E2F\u5206\u9928\u66F8\u5EAB&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;BPCL - \\u5317\\u6E2F\\u5206\\u9928\\u66F8\\u5EAB&quot; , &quot;'&quot; , &quot;, 183, true)&quot;); 
window.dlocation.add(623,1,&quot;cbook - cbook&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;cbook - cbook&quot; , &quot;'&quot; , &quot;, 623, true)&quot;); 
window.dlocation.add(624,1,&quot;circd - circd&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;circd - circd&quot; , &quot;'&quot; , &quot;, 624, true)&quot;); 
window.dlocation.add(683,1,&quot;clp - clp&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;clp - clp&quot; , &quot;'&quot; , &quot;, 683, true)&quot;); 
window.dlocation.add(170,1,&quot;CM - \u5317\u6E2F\u9644\u8A2D\u91AB\u9662&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;CM - \\u5317\\u6E2F\\u9644\\u8A2D\\u91AB\\u9662&quot; , &quot;'&quot; , &quot;, 170, true)&quot;); 
window.dlocation.add(211,170,&quot;BMHL - \u5317\u6E2F\u9644\u8A2D\u91AB\u9662\u5716\u66F8\u5BA4&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;BMHL - \\u5317\\u6E2F\\u9644\\u8A2D\\u91AB\\u9662\\u5716\\u66F8\\u5BA4&quot; , &quot;'&quot; , &quot;, 211, true)&quot;); 
window.dlocation.add(363,1,&quot;CMUL - \u4E2D\u570B\u91AB\u85E5\u5927\u5B78\u5716\u66F8\u9928 - CMUL - \u4E2D\u570B\u91AB\u85E5\u5927\u5B78\u5716\u66F8\u9928&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;CMUL - \\u4E2D\\u570B\\u91AB\\u85E5\\u5927\\u5B78\\u5716\\u66F8\\u9928 - CMUL - \\u4E2D\\u570B\\u91AB\\u85E5\\u5927\\u5B78\\u5716\\u66F8\\u9928&quot; , &quot;'&quot; , &quot;, 363, true)&quot;); 
window.dlocation.add(171,1,&quot;CU - \u53F0\u4E2D\u7E3D\u9928&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;CU - \\u53F0\\u4E2D\\u7E3D\\u9928&quot; , &quot;'&quot; , &quot;, 171, true)&quot;); 
window.dlocation.add(603,171,&quot;CUAV - \u53F0\u4E2D\u7E3D\u9928\u8996\u807D\u5340(\u7368\u7ACB\u5C08\u5340)&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;CUAV - \\u53F0\\u4E2D\\u7E3D\\u9928\\u8996\\u807D\\u5340(\\u7368\\u7ACB\\u5C08\\u5340)&quot; , &quot;'&quot; , &quot;, 603, true)&quot;); 
window.dlocation.add(217,171,&quot;MAVN - \u53F0\u4E2D\u7E3D\u9928\u8996\u807D\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MAVN - \\u53F0\\u4E2D\\u7E3D\\u9928\\u8996\\u807D\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 217, true)&quot;); 
window.dlocation.add(218,171,&quot;MAVR - \u53F0\u4E2D\u7E3D\u9928\u8996\u807D\u5340&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MAVR - \\u53F0\\u4E2D\\u7E3D\\u9928\\u8996\\u807D\\u5340&quot; , &quot;'&quot; , &quot;, 218, true)&quot;); 
window.dlocation.add(219,171,&quot;MCAT - \u53F0\u4E2D\u7E3D\u9928\u6280\u670D\u7D44&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MCAT - \\u53F0\\u4E2D\\u7E3D\\u9928\\u6280\\u670D\\u7D44&quot; , &quot;'&quot; , &quot;, 219, true)&quot;); 
window.dlocation.add(220,171,&quot;MCBS - \u53F0\u4E2D\u7E3D\u9928\u5BC6\u9589\u66F8\u5EAB&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MCBS - \\u53F0\\u4E2D\\u7E3D\\u9928\\u5BC6\\u9589\\u66F8\\u5EAB&quot; , &quot;'&quot; , &quot;, 220, true)&quot;); 
window.dlocation.add(221,171,&quot;MCIR - \u53F0\u4E2D\u7E3D\u9928\u6D41\u901A\u6AC3\u6AAF&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MCIR - \\u53F0\\u4E2D\\u7E3D\\u9928\\u6D41\\u901A\\u6AC3\\u6AAF&quot; , &quot;'&quot; , &quot;, 221, true)&quot;); 
window.dlocation.add(244,171,&quot;MCRA - \u53F0\u4E2D\u7E3D\u9928\u6559\u5E2B\u6307\u5B9A\u53C3\u8003\u66F8(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MCRA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u6559\\u5E2B\\u6307\\u5B9A\\u53C3\\u8003\\u66F8(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 244, true)&quot;); 
window.dlocation.add(245,171,&quot;MCSS - \u53F0\u4E2D\u7E3D\u9928B3\u88DD\u8A02\u671F\u520A\u5340&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MCSS - \\u53F0\\u4E2D\\u7E3D\\u9928B3\\u88DD\\u8A02\\u671F\\u520A\\u5340&quot; , &quot;'&quot; , &quot;, 245, true)&quot;); 
window.dlocation.add(246,171,&quot;MDIA - \u53F0\u4E2D\u7E3D\u9928\u535A\u78A9\u58EB\u8AD6\u6587\u5340&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MDIA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u535A\\u78A9\\u58EB\\u8AD6\\u6587\\u5340&quot; , &quot;'&quot; , &quot;, 246, true)&quot;); 
window.dlocation.add(248,171,&quot;MEAS - \u53F0\u4E2D\u7E3D\u9928\u63A2\u7D22\u53F2\u61F7\u54F2\u4E4B\u8DEF\u5C08\u66F8\u5340&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MEAS - \\u53F0\\u4E2D\\u7E3D\\u9928\\u63A2\\u7D22\\u53F2\\u61F7\\u54F2\\u4E4B\\u8DEF\\u5C08\\u66F8\\u5340&quot; , &quot;'&quot; , &quot;, 248, true)&quot;); 
window.dlocation.add(250,171,&quot;MEXM - \u53F0\u4E2D\u7E3D\u9928\u570B\u8003\u66F8\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MEXM - \\u53F0\\u4E2D\\u7E3D\\u9928\\u570B\\u8003\\u66F8\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 250, true)&quot;); 
window.dlocation.add(251,171,&quot;MFGA - \u53F0\u4E2D\u7E3D\u9928\u672C\u6821\u6559\u8077\u512A\u826F\u6559\u6750\u5340&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MFGA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u672C\\u6821\\u6559\\u8077\\u512A\\u826F\\u6559\\u6750\\u5340&quot; , &quot;'&quot; , &quot;, 251, true)&quot;); 
window.dlocation.add(252,171,&quot;MFPA - \u53F0\u4E2D\u7E3D\u9928\u672C\u6821\u6559\u5E2B\u5347\u7B49\u8CC7\u6599\u5340&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MFPA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u672C\\u6821\\u6559\\u5E2B\\u5347\\u7B49\\u8CC7\\u6599\\u5340&quot; , &quot;'&quot; , &quot;, 252, true)&quot;); 
window.dlocation.add(253,171,&quot;MFSA - \u53F0\u4E2D\u7E3D\u9928\u672C\u6821\u6559\u8077\u8AD6\u8457&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MFSA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u672C\\u6821\\u6559\\u8077\\u8AD6\\u8457&quot; , &quot;'&quot; , &quot;, 253, true)&quot;); 
window.dlocation.add(256,171,&quot;MHME - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u5B78\u502B\u7406(\u9650\u9928\u5167\u95B1\u89BD)\u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u5B78\u502B\u7406(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MHME - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u5B78\\u502B\\u7406(\\u9650\\u9928\\u5167\\u95B1\\u89BD)\\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u5B78\\u502B\\u7406(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 256, true)&quot;); 
window.dlocation.add(257,171,&quot;MHMH - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u7642\u53F2(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MHMH - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u7642\\u53F2(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 257, true)&quot;); 
window.dlocation.add(258,171,&quot;MHML - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u5B78\u6CD5\u5F8B(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MHML - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u5B78\\u6CD5\\u5F8B(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 258, true)&quot;); 
window.dlocation.add(259,171,&quot;MHQC - \u53F0\u4E2D\u7E3D\u9928\u4EBA\u6587\u5C08\u66F8\u5340-\u91AB\u7642\u54C1\u8CEA(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MHQC - \\u53F0\\u4E2D\\u7E3D\\u9928\\u4EBA\\u6587\\u5C08\\u66F8\\u5340-\\u91AB\\u7642\\u54C1\\u8CEA(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 259, true)&quot;); 
window.dlocation.add(261,171,&quot;MNBR - \u53F0\u4E2D\u7E3D\u9928\u65B0\u66F8\u5C55\u793A\u5340&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MNBR - \\u53F0\\u4E2D\\u7E3D\\u9928\\u65B0\\u66F8\\u5C55\\u793A\\u5340&quot; , &quot;'&quot; , &quot;, 261, true)&quot;); 
window.dlocation.add(262,171,&quot;MPAA - \u53F0\u4E2D\u7E3D\u9928\u7E3E\u6548\u66A8\u7372\u734E\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MPAA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7E3E\\u6548\\u66A8\\u7372\\u734E\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 262, true)&quot;); 
window.dlocation.add(263,171,&quot;MPBL - \u53F0\u4E2D\u7E3D\u9928PBL\u5C08\u66F8\u5340(\u9650\u9928\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MPBL - \\u53F0\\u4E2D\\u7E3D\\u9928PBL\\u5C08\\u66F8\\u5340(\\u9650\\u9928\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 263, true)&quot;); 
window.dlocation.add(265,171,&quot;MREF - \u53F0\u4E2D\u7E3D\u9928\u53C3\u8003\u5BA4&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MREF - \\u53F0\\u4E2D\\u7E3D\\u9928\\u53C3\\u8003\\u5BA4&quot; , &quot;'&quot; , &quot;, 265, true)&quot;); 
window.dlocation.add(266,171,&quot;MRSS - \u53F0\u4E2D\u7E3D\u9928\u95B1\u89BD\u7D44&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MRSS - \\u53F0\\u4E2D\\u7E3D\\u9928\\u95B1\\u89BD\\u7D44&quot; , &quot;'&quot; , &quot;, 266, true)&quot;); 
window.dlocation.add(267,171,&quot;MS-C - \u53F0\u4E2D\u7E3D\u9928\u671F\u520A\u8907\u672C\u6AC3&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MS-C - \\u53F0\\u4E2D\\u7E3D\\u9928\\u671F\\u520A\\u8907\\u672C\\u6AC3&quot; , &quot;'&quot; , &quot;, 267, true)&quot;); 
window.dlocation.add(270,171,&quot;MSER - \u53F0\u4E2D\u7E3D\u9928\u671F\u520A\u5340&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MSER - \\u53F0\\u4E2D\\u7E3D\\u9928\\u671F\\u520A\\u5340&quot; , &quot;'&quot; , &quot;, 270, true)&quot;); 
window.dlocation.add(271,171,&quot;MSPA - \u53F0\u4E2D\u7E3D\u9928\u7814\u7A76\u8A08\u5283\u5C08\u66F8&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MSPA - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7814\\u7A76\\u8A08\\u5283\\u5C08\\u66F8&quot; , &quot;'&quot; , &quot;, 271, true)&quot;); 
window.dlocation.add(272,171,&quot;MSPB - \u4E2D\u91AB\u91AB\u53F2\u6587\u737B\u5BA4(\u9650\u6240\u5167\u95B1\u89BD)&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MSPB - \\u4E2D\\u91AB\\u91AB\\u53F2\\u6587\\u737B\\u5BA4(\\u9650\\u6240\\u5167\\u95B1\\u89BD)&quot; , &quot;'&quot; , &quot;, 272, true)&quot;); 
window.dlocation.add(273,171,&quot;MSPC - \u53F0\u4E2D\u7E3D\u9928\u7279\u85CF\u5BA4&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MSPC - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7279\\u85CF\\u5BA4&quot; , &quot;'&quot; , &quot;, 273, true)&quot;); 
window.dlocation.add(274,171,&quot;MSTK - \u53F0\u4E2D\u7E3D\u9928\u66F8\u5EAB&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MSTK - \\u53F0\\u4E2D\\u7E3D\\u9928\\u66F8\\u5EAB&quot; , &quot;'&quot; , &quot;, 274, true)&quot;); 
window.dlocation.add(275,171,&quot;MYBK - \u53F0\u4E2D\u7E3D\u9928\u53C3\u8003\u58C1\u6AC3&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;MYBK - \\u53F0\\u4E2D\\u7E3D\\u9928\\u53C3\\u8003\\u58C1\\u6AC3&quot; , &quot;'&quot; , &quot;, 275, true)&quot;); 
window.dlocation.add(1103,171,&quot;new item 7 - new item 7&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;new item 7 - new item 7&quot; , &quot;'&quot; , &quot;, 1103, true)&quot;); 
window.dlocation.add(276,171,&quot;ONLN - \u53F0\u4E2D\u7E3D\u9928\u7DDA\u4E0A\u8CC7\u6599&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;ONLN - \\u53F0\\u4E2D\\u7E3D\\u9928\\u7DDA\\u4E0A\\u8CC7\\u6599&quot; , &quot;'&quot; , &quot;, 276, true)&quot;); 
window.dlocation.add(3,1,&quot;e-resources - \u96FB\u5B50\u8CC7\u6E90&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;e-resources - \\u96FB\\u5B50\\u8CC7\\u6E90&quot; , &quot;'&quot; , &quot;, 3, true)&quot;); 
window.dlocation.add(345,1,&quot;EB-P - EB-P&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;EB-P - EB-P&quot; , &quot;'&quot; , &quot;, 345, true)&quot;); 
window.dlocation.add(648,1,&quot;elect - elect&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;elect - elect&quot; , &quot;'&quot; , &quot;, 648, true)&quot;); 
window.dlocation.add(343,1,&quot;H-EQ - H-EQ&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;H-EQ - H-EQ&quot; , &quot;'&quot; , &quot;, 343, true)&quot;); 
window.dlocation.add(344,1,&quot;H-MR - H-MR&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;H-MR - H-MR&quot; , &quot;'&quot; , &quot;, 344, true)&quot;); 
window.dlocation.add(543,1,&quot;L - L&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;L - L&quot; , &quot;'&quot; , &quot;, 543, true)&quot;); 
window.dlocation.add(863,1,&quot;L40 - L40&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;L40 - L40&quot; , &quot;'&quot; , &quot;, 863, true)&quot;); 
window.dlocation.add(1023,1,&quot;LB \u5716\u66F8\u7E3D\u9928 - LB \u5716\u66F8\u7E3D\u9928&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;LB \\u5716\\u66F8\\u7E3D\\u9928 - LB \\u5716\\u66F8\\u7E3D\\u9928&quot; , &quot;'&quot; , &quot;, 1023, true)&quot;); 
window.dlocation.add(323,1,&quot;LB-S - LB-S&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;LB-S - LB-S&quot; , &quot;'&quot; , &quot;, 323, true)&quot;); 
window.dlocation.add(173,1,&quot;LE - \u8A9E\u6587\u6559\u5B78\u4E2D\u5FC3&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;LE - \\u8A9E\\u6587\\u6559\\u5B78\\u4E2D\\u5FC3&quot; , &quot;'&quot; , &quot;, 173, true)&quot;); 
window.dlocation.add(280,173,&quot;LEGL - \u8A9E\u6587\u6559\u5B78\u4E2D\u5FC3\u5716\u66F8\u5BA4&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;LEGL - \\u8A9E\\u6587\\u6559\\u5B78\\u4E2D\\u5FC3\\u5716\\u66F8\\u5BA4&quot; , &quot;'&quot; , &quot;, 280, true)&quot;); 
window.dlocation.add(523,1,&quot;LIB - LIB&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;LIB - LIB&quot; , &quot;'&quot; , &quot;, 523, true)&quot;); 
window.dlocation.add(423,1,&quot;new item 1 - new item 1&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;new item 1 - new item 1&quot; , &quot;'&quot; , &quot;, 423, true)&quot;); 
window.dlocation.add(484,423,&quot;new item 3 - new item 3&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;new item 3 - new item 3&quot; , &quot;'&quot; , &quot;, 484, true)&quot;); 
window.dlocation.add(1283,1,&quot;new item 10 - new item 10&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;new item 10 - new item 10&quot; , &quot;'&quot; , &quot;, 1283, true)&quot;); 
window.dlocation.add(1323,1,&quot;new item 12 - new item 12&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;new item 12 - new item 12&quot; , &quot;'&quot; , &quot;, 1323, true)&quot;); 
window.dlocation.add(1343,1,&quot;new item 13 - new item 13&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;new item 13 - new item 13&quot; , &quot;'&quot; , &quot;, 1343, true)&quot;); 
window.dlocation.add(1344,1,&quot;new item 14 - new item 14&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;new item 14 - new item 14&quot; , &quot;'&quot; , &quot;, 1344, true)&quot;); 
window.dlocation.add(1264,1,&quot;new item 16 - new item 16&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;new item 16 - new item 16&quot; , &quot;'&quot; , &quot;, 1264, true)&quot;); 
window.dlocation.add(483,1,&quot;new item 2 - new item 2&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;new item 2 - new item 2&quot; , &quot;'&quot; , &quot;, 483, true)&quot;); 
window.dlocation.add(1425,1,&quot;new item 20 - new item 20&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;new item 20 - new item 20&quot; , &quot;'&quot; , &quot;, 1425, true)&quot;); 
window.dlocation.add(943,1,&quot;new item 4 - new item 4&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;new item 4 - new item 4&quot; , &quot;'&quot; , &quot;, 943, true)&quot;); 
window.dlocation.add(963,1,&quot;new item 5 - new item 5&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;new item 5 - new item 5&quot; , &quot;'&quot; , &quot;, 963, true)&quot;); 
window.dlocation.add(1063,1,&quot;new item 6 - \u82F1\u624D\u6821\u5340&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;new item 6 - \\u82F1\\u624D\\u6821\\u5340&quot; , &quot;'&quot; , &quot;, 1063, true)&quot;); 
window.dlocation.add(1243,1,&quot;new item 8 - new item 8&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;new item 8 - new item 8&quot; , &quot;'&quot; , &quot;, 1243, true)&quot;); 
window.dlocation.add(1263,1,&quot;new item 9 - new item 9&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;new item 9 - new item 9&quot; , &quot;'&quot; , &quot;, 1263, true)&quot;); 
window.dlocation.add(1043,1,&quot;NPTU - NPTU&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;NPTU - NPTU&quot; , &quot;'&quot; , &quot;, 1043, true)&quot;); 
window.dlocation.add(503,1,&quot;OUK - OUK&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;OUK - OUK&quot; , &quot;'&quot; , &quot;, 503, true)&quot;); 
window.dlocation.add(174,1,&quot;PT - \u57F9\u5FB7\u91AB\u9662&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;PT - \\u57F9\\u5FB7\\u91AB\\u9662&quot; , &quot;'&quot; , &quot;, 174, true)&quot;); 
window.dlocation.add(1303,174,&quot;new item 11 - new item 11&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;new item 11 - new item 11&quot; , &quot;'&quot; , &quot;, 1303, true)&quot;); 
window.dlocation.add(1363,174,&quot;new item 17 - new item 17&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;new item 17 - new item 17&quot; , &quot;'&quot; , &quot;, 1363, true)&quot;); 
window.dlocation.add(283,174,&quot;PTGL - \u57F9\u5FB7\u91AB\u9662\u5716\u66F8\u5340&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;PTGL - \\u57F9\\u5FB7\\u91AB\\u9662\\u5716\\u66F8\\u5340&quot; , &quot;'&quot; , &quot;, 283, true)&quot;); 
window.dlocation.add(645,1,&quot;ptext - ptext&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;ptext - ptext&quot; , &quot;'&quot; , &quot;, 645, true)&quot;); 
window.dlocation.add(1083,1,&quot;SB3 - SB3&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;SB3 - SB3&quot; , &quot;'&quot; , &quot;, 1083, true)&quot;); 
window.dlocation.add(324,1,&quot;T-P - T-P&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;T-P - T-P&quot; , &quot;'&quot; , &quot;, 324, true)&quot;); 
window.dlocation.add(1403,1,&quot;TBBK - TBBK&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;TBBK - TBBK&quot; , &quot;'&quot; , &quot;, 1403, true)&quot;); 
window.dlocation.add(175,1,&quot;TH - \u53F0\u5317\u5206\u9662&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;TH - \\u53F0\\u5317\\u5206\\u9662&quot; , &quot;'&quot; , &quot;, 175, true)&quot;); 
window.dlocation.add(284,175,&quot;THGL - \u53F0\u5317\u5206\u9662\u5716\u66F8\u5340&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;THGL - \\u53F0\\u5317\\u5206\\u9662\\u5716\\u66F8\\u5340&quot; , &quot;'&quot; , &quot;, 284, true)&quot;); 
window.dlocation.add(285,175,&quot;THPA - \u53F0\u5317\u5206\u9662\u671F\u520A\u5340&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;THPA - \\u53F0\\u5317\\u5206\\u9662\\u671F\\u520A\\u5340&quot; , &quot;'&quot; , &quot;, 285, true)&quot;); 
window.dlocation.add(325,1,&quot;W-P - W-P&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;W-P - W-P&quot; , &quot;'&quot; , &quot;, 325, true)&quot;); 
window.dlocation.add(176,1,&quot;YH - \u8C50\u539F\u5206\u9662&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;YH - \\u8C50\\u539F\\u5206\\u9662&quot; , &quot;'&quot; , &quot;, 176, true)&quot;); 
window.dlocation.add(1423,176,&quot;new item 18 - new item 18&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;new item 18 - new item 18&quot; , &quot;'&quot; , &quot;, 1423, true)&quot;); 
window.dlocation.add(1424,176,&quot;new item 19 - new item 19&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;new item 19 - new item 19&quot; , &quot;'&quot; , &quot;, 1424, true)&quot;); 
window.dlocation.add(286,176,&quot;YHGL - \u8C50\u539F\u5206\u9662\u5716\u66F8\u5340&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;YHGL - \\u8C50\\u539F\\u5206\\u9662\\u5716\\u66F8\\u5340&quot; , &quot;'&quot; , &quot;, 286, true)&quot;); 
window.dlocation.add(983,1,&quot;z3llc - z3llc&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;z3llc - z3llc&quot; , &quot;'&quot; , &quot;, 983, true)&quot;); 
window.dlocation.add(647,1,&quot;z6bkf - z6bkf&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;z6bkf - z6bkf&quot; , &quot;'&quot; , &quot;, 647, true)&quot;); 
window.dlocation.add(646,1,&quot;zd1a2 - zd1a2&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;zd1a2 - zd1a2&quot; , &quot;'&quot; , &quot;, 646, true)&quot;); 
window.dlocation.add(663,1,&quot;zd1e - zd1e&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;zd1e - zd1e&quot; , &quot;'&quot; , &quot;, 663, true)&quot;); 
window.dlocation.add(644,1,&quot;zdlf - zdlf&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;zdlf - zdlf&quot; , &quot;'&quot; , &quot;, 644, true)&quot;); 
window.dlocation.add(403,1,&quot;\u4E94\u6A13\u6F2B\u756B\u66F8\u5C08\u5340 - \u4E94\u6A13\u6F2B\u756B\u66F8\u5C08\u5340&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;\\u4E94\\u6A13\\u6F2B\\u756B\\u66F8\\u5C08\\u5340 - \\u4E94\\u6A13\\u6F2B\\u756B\\u66F8\\u5C08\\u5340&quot; , &quot;'&quot; , &quot;, 403, true)&quot;); 
window.dlocation.add(563,1,&quot;\u5BE6\u9AD4\u9928\u85CF - \u5BE6\u9AD4\u9928\u85CF&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;\\u5BE6\\u9AD4\\u9928\\u85CF - \\u5BE6\\u9AD4\\u9928\\u85CF&quot; , &quot;'&quot; , &quot;, 563, true)&quot;); 
window.dlocation.add(1383,1,&quot;\u7D9C\u5408\u66F8\u5EAB - \u7D9C\u5408\u66F8\u5EAB&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;\\u7D9C\\u5408\\u66F8\\u5EAB - \\u7D9C\\u5408\\u66F8\\u5EAB&quot; , &quot;'&quot; , &quot;, 1383, true)&quot;); 
window.dlocation.add(383,1,&quot;\u827E\u8FEA\u8A0A\u5716\u66F8\u9928 - \u827E\u8FEA\u8A0A\u5716\u66F8\u9928&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;\\u827E\\u8FEA\\u8A0A\\u5716\\u66F8\\u9928 - \\u827E\\u8FEA\\u8A0A\\u5716\\u66F8\\u9928&quot; , &quot;'&quot; , &quot;, 383, true)&quot;); 
window.dlocation.add(1384,1,&quot;\u9644\u4E2D\u51FA\u7248\u7269\u5C08\u5340 - \u9644\u4E2D\u51FA\u7248\u7269\u5C08\u5340&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;\\u9644\\u4E2D\\u51FA\\u7248\\u7269\\u5C08\\u5340 - \\u9644\\u4E2D\\u51FA\\u7248\\u7269\\u5C08\\u5340&quot; , &quot;'&quot; , &quot;, 1384, true)&quot;); 
window.dlocation.add(443,1,&quot;\u9F8D\u83EF\u79D1\u6280\u5927\u5B78\u5716\u66F8\u9928 - \u9F8D\u83EF\u79D1\u6280\u5927\u5B78\u5716\u66F8\u9928&quot;, &quot;javascript:window.dlocation.selectElement(&quot; , &quot;'&quot; , &quot;\\u9F8D\\u83EF\\u79D1\\u6280\\u5927\\u5B78\\u5716\\u66F8\\u9928 - \\u9F8D\\u83EF\\u79D1\\u6280\\u5927\\u5B78\\u5716\\u66F8\\u9928&quot; , &quot;'&quot; , &quot;, 443, true)&quot;); 
window.dlocation.selectElement = function(lname, id, hideTree) { 
document.getElementById(&quot; , &quot;'&quot; , &quot;location_0&quot; , &quot;'&quot; , &quot;).value = id; 
document.getElementById(&quot; , &quot;'&quot; , &quot;elementName&quot; , &quot;'&quot; , &quot;).value = lname; 
if(hideTree == true) changeStatus(&quot; , &quot;'&quot; , &quot;locationTree&quot; , &quot;'&quot; , &quot;); 
}; 
 document.getElementById(&quot; , &quot;'&quot; , &quot;locationArea&quot; , &quot;'&quot; , &quot;).innerHTML =  window.dlocation; 
  
  
  


						
				
			
			
				
					
						請輸入要修改的條碼號
						
條碼號

						
							
							

//&lt;![CDATA[

								document.getElementById(&quot;listField&quot;).focus();
							
//]]&gt;


						
						
						
						
					
				
			
		

		
			
				注意:若是修改的資料內容與原始內容相同，則不修改亦不顯示。
			

			
		
		

			
				
					
						
							
								
									
										排序條件:   
條碼號
分類號

										
									
								
								
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
500
 
			
		

		
			

        	

	






            




	jQuery.noConflict();
	var popNewBookSelect={
	          node_this:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,
			  code:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,       
			  name:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,
			  popUp : function(codePtr,namePtr,top,left,obj){//POPUP WINDOW------------------------------------------------------
			          (function($){
						  $(&quot;#popNewBookSelect&quot;).css({&quot; , &quot;'&quot; , &quot;top&quot; , &quot;'&quot; , &quot; : top ,&quot; , &quot;'&quot; , &quot;left&quot; , &quot;'&quot; , &quot; : left});//位置設定
						  popNewBookSelect.node_this  = $(obj);
						  popNewBookSelect.code  	   = codePtr;
						  popNewBookSelect.name  	   = namePtr;
				          //var dropdownSet = $(&quot; , &quot;'&quot; , &quot;#popNewBookSelect #L1dropdown&quot; , &quot;'&quot; , &quot;);
		                  popNewBookSelect.init();
					  })(jQuery);
			  },
              init : function(){
			          (function($){
						    var contextPath=trim($(&quot; , &quot;'&quot; , &quot;#popNewBookSelect #contextPath&quot; , &quot;'&quot; , &quot;).text());//取contextPath
			                $(&quot; , &quot;'&quot; , &quot;#popNewBookSelect img.clear&quot; , &quot;'&quot; , &quot;).attr(&quot;src&quot;,contextPath+&quot;images/clear.gif&quot;);
			                $(&quot; , &quot;'&quot; , &quot;#popNewBookSelect img.wait&quot; , &quot;'&quot; , &quot;).attr(&quot;src&quot;,contextPath+&quot;images/wait.gif&quot;);
			                //alert($(&quot; , &quot;'&quot; , &quot;#popNewBookSelect img.clear&quot; , &quot;'&quot; , &quot;).attr(&quot;src&quot;));
			                $(&quot;#popNewBookSelect input.button&quot;).val($(&quot; , &quot;'&quot; , &quot;#popNewBookSelect #save&quot; , &quot;'&quot; , &quot;).text());
			              	$(&quot;#popNewBookSelect&quot;).show();
					  })(jQuery);
              },
		      cancel : function() {//CANCEL----------------------------------------------------------------------------
					(function($){
							$(&quot;#popNewBookSelect&quot;).hide();
					})(jQuery);
			  },
		      checkBoxClear : function() {//checkBox Clear----------------------------------------------------------------------------
					(function($){
						$(&quot;#JQ_ItemCRS input.check&quot;).each(function(){
								$(this).attr(&quot;checked&quot;,false);
						});
						$(&quot;#rootCheck&quot;).attr(&quot;checked&quot;,false);
					})(jQuery);
			  },
		      confirm : function() {//run--------------------------------------------------------------------------
					(function($){
						$(&quot;#popNewBookSelect&quot;).hide();
						var contextPath=trim($(&quot; , &quot;'&quot; , &quot;#popNewBookSelect #contextPath&quot; , &quot;'&quot; , &quot;).text());//取contextPath
						var year =$(&quot; , &quot;'&quot; , &quot;#year  :selected&quot; , &quot;'&quot; , &quot;).text();
						var month=$(&quot; , &quot;'&quot; , &quot;#month :selected&quot; , &quot;'&quot; , &quot;).text();
					    var i=0;
					    var flag=&quot;n&quot;;
					    var selectItems=&quot;&quot;;
						$(&quot;#JQ_ItemCRS input.check&quot;).each(function(){
								//alert(i+&quot; - &quot;+$(this).attr(&quot;checked&quot;));
								var select=$(this).attr(&quot;checked&quot;);
								if(select==true){
									//alert(i+&quot; - &quot;+$(this).parent().parent().find(&quot;td[className=itemId]&quot;).text());
									var itemId=trim($(this).parent().parent().find(&quot;td[className=itemId]&quot;).text());
									if(flag==&quot;y&quot;) selectItems=selectItems+&quot;,&quot;;
									selectItems=selectItems+itemId;
									flag=&quot;y&quot;;
								}
								i++;
						});
						//alert(&quot;REQ=&quot;+selectItems);
						//alert(&quot;year=&quot;+year);
						//alert(&quot;month=&quot;+month);
						popNewBookSelect.checkBoxClear();
				        $.ajax({ 
							    type: &quot;POST&quot;, 
							    url: contextPath+&quot;/kidMain&quot;, 
							    dataType: &quot;text&quot;, 
							    data: {method:&quot;setNewBooks&quot;,selectItems:selectItems,year:year,month:month}, 
							    success: function(html) {
										//alert(&quot;RES=&quot;+html);
										var success=$(&quot; , &quot;'&quot; , &quot;#popNewBookSelect #success&quot; , &quot;'&quot; , &quot;).text();
										alert(success);
							    }, 
							    error: function (XMLHttpRequest, textStatus, errorThrown) { 
							    	alert(XMLHttpRequest.responseText);
							    } 
						});
					})(jQuery);
			  }
	}	  


&lt;!--
	#popNewBookSelect {
		position: absolute;
		left: 250px;
		top: 200px;
		z-index: 1;
	}
	#popNewBookSelect .pop_foreground {
	z-index:3;
	left: 21px;
	background-color: #FFFFFF;
	position: absolute;
	width: 342px;
	border-top-width: 1px;
	border-right-width: 1px;
	border-bottom-width: 1px;
	border-left-width: 1px;
	border-top-style: solid;
	border-right-style: solid;
	border-bottom-style: solid;
	border-left-style: solid;
	border-top-color: #84A9D8;
	border-right-color: #84A9D8;
	border-bottom-color: #84A9D8;
	border-left-color: #84A9D8;
	}
	#popNewBookSelect .pop_foreground .pop_cancel {
		position: absolute;
		top: 6px;
		right: 10px;
	}
	#popNewBookSelect .pop_foreground .pop_cancel a {
		color: #FF0000;
		text-decoration: none;
	}
	#popNewBookSelect .pop_foreground  .pop_header {
	color: #FFFFFF;
	background-color: #84A9D8;
	margin-top: 0px;
	margin-bottom: 0px;
	}
	#popNewBookSelect .pop_foreground  .pop_header p {
		margin-top: 0px;
		margin-bottom: 0px;
		font-size: 12pt;
		line-height: 2em;
		font-weight: bold;
	}
	#popNewBookSelect .pop_foreground .pop_content {
		width: 100%;
		margin-top: 0px;
		margin-bottom: 0px;
		border: 1px solid #666666;
		background: #FFFFFF;
	}
	#popNewBookSelect .pop_foreground .pop_content a {
		text-decoration: none;
		font-size: 12pt;
		line-height: 1em;
		color: #000000;
		display: block;
	}
	#popNewBookSelect .pop_foreground .pop_content a:hover {
		text-decoration: none;
		font-size: 12pt;
		line-height: 1em;
		color: #FFFFFF;
		background: #a69f00;
	}
	#popNewBookSelect .pop_foreground  .pop_content p {
		margin-top: 0px;
		margin-bottom: 0px;
		font-size: 12pt;
		line-height: 1em;
	}
	#popNewBookSelect .pop_foreground .pop_footer {
		background: #FFA100;
		text-align: center;
		font-size: 12pt;
		line-height: 1em;
		color: #993300;
	}

	#popNewBookSelect #detailFormContainer {
	width: 322px;
	padding-top: 2px;
	padding-right: 10px;
	padding-bottom: 12px;
	padding-left: 10px;
	}
	#popNewBookSelect #detailFormContainer br {
			clear: both;
	}
	#popNewBookSelect #detailFormContainer #cascadingDropdowns div {
		  float: left;
		  margin-right: 10px;
	}
	#popNewBookSelect #detailFormContainer label {
			float: left;
			margin-right: 10px;
			color: #FFFFFF;
			font: 24px &quot;標楷體&quot;;
	}
#popNewBookSelect #confirm {
	text-align: right;
	margin-left: 225px;
	margin-top: 20px;
}
#popNewBookSelect #confirm .button {
	background-color: #84A9D8;
	margin: 0px;
	border: 1px outset #CCC;
}
-->



		/inspireapp/
		
		
	    
	    	
	        
	          	
	        
	        
      			
                    
                    
                          
                          	
							      
									  
									        2012
									        2011
									        2010
									        2009
									        2008
									        2007
									        2006
									        2005
									        2004
									        2003
									        2002
									        2001
									        2000
								      
							      
							           
									  
									          01
									          02
									          03
									          04
									          05
									          06
									          07
									          08
									          09
									          10
									          11
									          12
								      
							      
							  
	                          
 		    						
							  
                          
                    
                    
                
            
	    

	
	

        	




	jQuery.noConflict();
	var popGoodBookSelect={
	          node_this:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,
			  code:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,       
			  name:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,
			  popUp : function(codePtr,namePtr,top,left,obj){//POPUP WINDOW------------------------------------------------------
			          (function($){
						  $(&quot;#popGoodBookSelect&quot;).css({&quot; , &quot;'&quot; , &quot;top&quot; , &quot;'&quot; , &quot; : top ,&quot; , &quot;'&quot; , &quot;left&quot; , &quot;'&quot; , &quot; : left});//位置設定
						  popGoodBookSelect.node_this  = $(obj);
						  popGoodBookSelect.code  	   = codePtr;
						  popGoodBookSelect.name  	   = namePtr;
		                  popGoodBookSelect.init();
					  })(jQuery);
			  },
              init : function(){
			          (function($){
						    var contextPath=trim($(&quot; , &quot;'&quot; , &quot;#popGoodBookSelect #contextPath&quot; , &quot;'&quot; , &quot;).text());//取contextPath
			                $(&quot; , &quot;'&quot; , &quot;#popGoodBookSelect img.clear&quot; , &quot;'&quot; , &quot;).attr(&quot;src&quot;,contextPath+&quot;images/clear.gif&quot;);
			                $(&quot; , &quot;'&quot; , &quot;#popGoodBookSelect img.wait&quot; , &quot;'&quot; , &quot;).attr(&quot;src&quot;,contextPath+&quot;images/wait.gif&quot;);
					        $.ajax({ 
								    type: &quot;POST&quot;, 
								    url: contextPath+&quot;/kidMain&quot;, 
								    dataType: &quot;json&quot;, 
								    data: {method:&quot;getGoodBookItemCode&quot;,language:&quot;4&quot;,ignore:&quot;00&quot;}, 
								    success: function(data) {
										  //alert(&quot;RES=&quot;+data);
						              	  $(&quot;#popGoodBookSelect #L1DropDown&quot;).loadSelect(data);
								    }, 
								    error: function (XMLHttpRequest, textStatus, errorThrown) { 
								    	alert(XMLHttpRequest.responseText);
								    } 
							});
			                $(&quot;#popGoodBookSelect input.button&quot;).val($(&quot; , &quot;'&quot; , &quot;#popGoodBookSelect #save&quot; , &quot;'&quot; , &quot;).text());
			              	$(&quot;#popGoodBookSelect&quot;).show();
					  })(jQuery);
              },
		      cancel : function() {//CANCEL----------------------------------------------------------------------------
					(function($){
							$(&quot;#popGoodBookSelect&quot;).hide();
					})(jQuery);
			  },
		      checkBoxClear : function() {//checkBox Clear----------------------------------------------------------------------------
					(function($){
						$(&quot;#JQ_ItemCRS input.check&quot;).each(function(){
								$(this).attr(&quot;checked&quot;,false);
						});
						$(&quot;#rootCheck&quot;).attr(&quot;checked&quot;,false);
					})(jQuery);
			  },
		      confirm : function() {//run--------------------------------------------------------------------------
					(function($){
						$(&quot;#popGoodBookSelect&quot;).hide();
						var contextPath=trim($(&quot; , &quot;'&quot; , &quot;#popGoodBookSelect #contextPath&quot; , &quot;'&quot; , &quot;).text());//取contextPath
						//var id =$(&quot; , &quot;'&quot; , &quot;#popGoodBookSelect #topicId  :selected&quot; , &quot;'&quot; , &quot;).val();
						var id=$(&quot; , &quot;'&quot; , &quot;#popGoodBookSelect #L1DropDown&quot; , &quot;'&quot; , &quot;).val();
					    var i=0;
					    var flag=&quot;n&quot;;
					    var selectItems=&quot;&quot;;
						$(&quot;#JQ_ItemCRS input.check&quot;).each(function(){
								//alert(i+&quot; - &quot;+$(this).attr(&quot;checked&quot;));
								var select=$(this).attr(&quot;checked&quot;);
								if(select==true){
									//alert(i+&quot; - &quot;+$(this).parent().parent().find(&quot;td[className=itemId]&quot;).text());
									var itemId=trim($(this).parent().parent().find(&quot;td[className=itemId]&quot;).text());
									if(flag==&quot;y&quot;) selectItems=selectItems+&quot;,&quot;;
									selectItems=selectItems+itemId;
									flag=&quot;y&quot;;
								}
								i++;
						});
						//alert(&quot;good book REQ=&quot;+selectItems);
						//alert(&quot;good book topc-id=&quot;+id);
						popGoodBookSelect.checkBoxClear();
				        $.ajax({ 
							    type: &quot;POST&quot;, 
							    url: contextPath+&quot;/kidMain&quot;, 
							    dataType: &quot;text&quot;, 
							    data: {method:&quot;setGoodBooks&quot;,selectItems:selectItems,id:id}, 
							    success: function(html) {
										//alert(&quot;RES=&quot;+html);
										var success=$(&quot; , &quot;'&quot; , &quot;#popGoodBookSelect #success&quot; , &quot;'&quot; , &quot;).text();
										alert(success);
							    }, 
							    error: function (XMLHttpRequest, textStatus, errorThrown) { 
							    	alert(XMLHttpRequest.responseText);
							    } 
						});
					})(jQuery);
			  }
	}	  


&lt;!--
	#popGoodBookSelect {
		position: absolute;
		left: 250px;
		top: 200px;
		z-index: 1;
	}
	#popGoodBookSelect .pop_foreground {
	z-index:3;
	left: 21px;
	background-color: #FFFFFF;
	position: absolute;
	width: 342px;
	border-top-width: 1px;
	border-right-width: 1px;
	border-bottom-width: 1px;
	border-left-width: 1px;
	border-top-style: solid;
	border-right-style: solid;
	border-bottom-style: solid;
	border-left-style: solid;
	border-top-color: #84A9D8;
	border-right-color: #84A9D8;
	border-bottom-color: #84A9D8;
	border-left-color: #84A9D8;
	}
	#popGoodBookSelect .pop_foreground .pop_cancel {
		position: absolute;
		top: 6px;
		right: 10px;
	}
	#popGoodBookSelect .pop_foreground .pop_cancel a {
		color: #FF0000;
		text-decoration: none;
	}
	#popGoodBookSelect .pop_foreground  .pop_header {
	color: #FFFFFF;
	background-color: #84A9D8;
	margin-top: 0px;
	margin-bottom: 0px;
	}
	#popGoodBookSelect .pop_foreground  .pop_header p {
		margin-top: 0px;
		margin-bottom: 0px;
		font-size: 12pt;
		line-height: 2em;
		font-weight: bold;
	}
	#popGoodBookSelect .pop_foreground .pop_content {
		width: 100%;
		margin-top: 0px;
		margin-bottom: 0px;
		border: 1px solid #666666;
		background: #FFFFFF;
	}
	#popGoodBookSelect .pop_foreground .pop_content a {
		text-decoration: none;
		font-size: 12pt;
		line-height: 1em;
		color: #000000;
		display: block;
	}
	#popGoodBookSelect .pop_foreground .pop_content a:hover {
		text-decoration: none;
		font-size: 12pt;
		line-height: 1em;
		color: #FFFFFF;
		background: #a69f00;
	}
	#popGoodBookSelect .pop_foreground  .pop_content p {
		margin-top: 0px;
		margin-bottom: 0px;
		font-size: 12pt;
		line-height: 1em;
	}
	#popGoodBookSelect .pop_foreground .pop_footer {
		background: #FFA100;
		text-align: center;
		font-size: 12pt;
		line-height: 1em;
		color: #993300;
	}

	#popGoodBookSelect #detailFormContainer {
	width: 322px;
	padding-top: 2px;
	padding-right: 10px;
	padding-bottom: 12px;
	padding-left: 10px;
	}
	#popGoodBookSelect #detailFormContainer br {
			clear: both;
	}
	#popGoodBookSelect #detailFormContainer #cascadingDropdowns div {
		  float: left;
		  margin-right: 10px;
	}
	#popGoodBookSelect #detailFormContainer label {
			float: left;
			margin-right: 10px;
			color: #FFFFFF;
			font: 24px &quot;標楷體&quot;;
	}
#popGoodBookSelect #confirm {
	text-align: right;
	margin-left: 225px;
	margin-top: 20px;
}
#popGoodBookSelect #confirm .button {
	background-color: #84A9D8;
	margin: 0px;
	border: 1px outset #CCC;
}
-->



		/inspireapp/
		
		
	    
	    	
	        
	          	Good Book Setup
	        
	        
      			
                    
                    
                          
                          	  
							      
	                              
							      
							  
	                          
 		    						
							  
                          
                    
                    
                
            
	    

	
	

        	




	jQuery.noConflict();
	var popTopicBookSelect={
	          node_this:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,
			  code:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,       
			  name:&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;,
			  popUp : function(codePtr,namePtr,top,left,obj){//POPUP WINDOW------------------------------------------------------
			          (function($){
						  $(&quot;#popTopicBookSelect&quot;).css({&quot; , &quot;'&quot; , &quot;top&quot; , &quot;'&quot; , &quot; : top ,&quot; , &quot;'&quot; , &quot;left&quot; , &quot;'&quot; , &quot; : left});//位置設定
						  popTopicBookSelect.node_this  = $(obj);
						  popTopicBookSelect.code  	   = codePtr;
						  popTopicBookSelect.name  	   = namePtr;
		                  popTopicBookSelect.init();
					  })(jQuery);
			  },
              init : function(){
			          (function($){
						    var contextPath=trim($(&quot; , &quot;'&quot; , &quot;#popTopicBookSelect #contextPath&quot; , &quot;'&quot; , &quot;).text());//取contextPath
			                $(&quot; , &quot;'&quot; , &quot;#popTopicBookSelect img.clear&quot; , &quot;'&quot; , &quot;).attr(&quot;src&quot;,contextPath+&quot;images/clear.gif&quot;);
			                $(&quot; , &quot;'&quot; , &quot;#popTopicBookSelect img.wait&quot; , &quot;'&quot; , &quot;).attr(&quot;src&quot;,contextPath+&quot;images/wait.gif&quot;);
					        $.ajax({ 
								    type: &quot;POST&quot;, 
								    url: contextPath+&quot;/kidMain&quot;, 
								    dataType: &quot;json&quot;, 
								    data: {method:&quot;getTopicBookMainItemCode&quot;,language:&quot;4&quot;}, 
								    success: function(data) {
										  //alert(&quot;RES=&quot;+data);
						              	  $(&quot;#popTopicBookSelect #L1DropDown&quot;).loadSelect(data);
						              	  popTopicBookSelect.L2_Dropdown();
								    }, 
								    error: function (XMLHttpRequest, textStatus, errorThrown) { 
								    	alert(XMLHttpRequest.responseText);
								    } 
							});
							//bind
					        $(&quot; , &quot;'&quot; , &quot;#popTopicBookSelect #L1dropdown&quot; , &quot;'&quot; , &quot;).change(function(){
						            popTopicBookSelect.L2_Dropdown();
					        });
			                $(&quot;#popTopicBookSelect input.button&quot;).val($(&quot; , &quot;'&quot; , &quot;#popTopicBookSelect #save&quot; , &quot;'&quot; , &quot;).text());
			              	$(&quot;#popTopicBookSelect&quot;).show();
					  })(jQuery);
              },
		      L2_Dropdown : function() {//LEVEL 2----------------------------------------------------------------------------------
					(function($){
				        var L1_value    = $(&quot; , &quot;'&quot; , &quot;#popTopicBookSelect #L1dropdown&quot; , &quot;'&quot; , &quot;).val();
				        var dropdownSet = $(&quot; , &quot;'&quot; , &quot;#popTopicBookSelect #L2dropdown&quot; , &quot;'&quot; , &quot;);
				        if (L1_value.length == 0) {
					          dropdownSet.attr(&quot;disabled&quot;,true);
					          dropdownSet.emptySelect();
				        }else {
					          dropdownSet.attr(&quot;disabled&quot;,false);
						      var contextPath=trim($(&quot; , &quot;'&quot; , &quot;#popTopicBookSelect #contextPath&quot; , &quot;'&quot; , &quot;).text());//取contextPath
					          $.ajax({ 
									type: &quot;POST&quot;, 
									url: contextPath+&quot;/kidMain&quot;, 
									dataType: &quot;json&quot;, 
									data: {method:&quot;getTopicBookSubItemCode&quot;,main_id:trim(L1_value),language:&quot;4&quot;}, 
								    success: function(data) {
						            	dropdownSet.loadSelect(data);
								    }, 
								    error: function (XMLHttpRequest, textStatus, errorThrown) { 
								    	//alert(XMLHttpRequest.responseText);
								    	alert(&quot;no data&quot;);
								    } 
							  });
				        }
					})(jQuery);
		      },
		      cancel : function() {//CANCEL----------------------------------------------------------------------------
					(function($){
							$(&quot;#popTopicBookSelect&quot;).hide();
					})(jQuery);
			  },
		      checkBoxClear : function() {//checkBox Clear----------------------------------------------------------------------------
					(function($){
						$(&quot;#JQ_ItemCRS input.check&quot;).each(function(){
								$(this).attr(&quot;checked&quot;,false);
						});
						$(&quot;#rootCheck&quot;).attr(&quot;checked&quot;,false);
					})(jQuery);
			  },
		      confirm : function() {//run--------------------------------------------------------------------------
					(function($){
							$(&quot;#popTopicBookSelect&quot;).hide();
							var contextPath=trim($(&quot; , &quot;'&quot; , &quot;#popTopicBookSelect #contextPath&quot; , &quot;'&quot; , &quot;).text());//取contextPath
						    var i=0;
						    var flag=&quot;n&quot;;
						    var selectItems=&quot;&quot;;
							$(&quot;#JQ_ItemCRS input.check&quot;).each(function(){
									//alert(i+&quot; - &quot;+$(this).attr(&quot;checked&quot;));
									var select=$(this).attr(&quot;checked&quot;);
									if(select==true){
										//alert(i+&quot; - &quot;+$(this).parent().parent().find(&quot;td[className=itemId]&quot;).text());
										var itemId=trim($(this).parent().parent().find(&quot;td[className=itemId]&quot;).text());
										if(flag==&quot;y&quot;) selectItems=selectItems+&quot;,&quot;;
										selectItems=selectItems+itemId;
										flag=&quot;y&quot;;
									}
									i++;
							});
							//alert(&quot;good book REQ=&quot;+selectItems);
					        var mainId =$(&quot;#popTopicBookSelect #L1dropdown&quot;).find(&quot; , &quot;'&quot; , &quot;:selected&quot; , &quot;'&quot; , &quot;).val();
					        var subId  =$(&quot;#popTopicBookSelect #L2dropdown&quot;).find(&quot; , &quot;'&quot; , &quot;:selected&quot; , &quot;'&quot; , &quot;).val();
					        //alert(mainId+&quot;/&quot;+subId);
							popTopicBookSelect.checkBoxClear();
					        $.ajax({ 
								    type: &quot;POST&quot;, 
								    url: contextPath+&quot;/kidMain&quot;, 
								    dataType: &quot;text&quot;, 
								    data: {method:&quot;setTopicBooks&quot;,selectItems:selectItems,mainId:mainId,subId:subId}, 
								    success: function(html) {
											//alert(&quot;RES=&quot;+html);
											var success=$(&quot; , &quot;'&quot; , &quot;#popTopicBookSelect #success&quot; , &quot;'&quot; , &quot;).text();
											alert(success);
								    }, 
								    error: function (XMLHttpRequest, textStatus, errorThrown) { 
								    	alert(XMLHttpRequest.responseText);
								    } 
							});
					})(jQuery);
			  }
	}	  


&lt;!--
	#popTopicBookSelect {
		position: absolute;
		left: 250px;
		top: 200px;
		z-index: 1;
	}
	#popTopicBookSelect .pop_foreground {
	z-index:3;
	left: 21px;
	background-color: #FFFFFF;
	position: absolute;
	width: 342px;
	border-top-width: 1px;
	border-right-width: 1px;
	border-bottom-width: 1px;
	border-left-width: 1px;
	border-top-style: solid;
	border-right-style: solid;
	border-bottom-style: solid;
	border-left-style: solid;
	border-top-color: #84A9D8;
	border-right-color: #84A9D8;
	border-bottom-color: #84A9D8;
	border-left-color: #84A9D8;
	}
	#popTopicBookSelect .pop_foreground .pop_cancel {
		position: absolute;
		top: 6px;
		right: 10px;
	}
	#popTopicBookSelect .pop_foreground .pop_cancel a {
		color: #FF0000;
		text-decoration: none;
	}
	#popTopicBookSelect .pop_foreground  .pop_header {
	color: #FFFFFF;
	background-color: #84A9D8;
	margin-top: 0px;
	margin-bottom: 0px;
	}
	#popTopicBookSelect .pop_foreground  .pop_header p {
		margin-top: 0px;
		margin-bottom: 0px;
		font-size: 12pt;
		line-height: 2em;
		font-weight: bold;
	}
	#popTopicBookSelect .pop_foreground .pop_content {
		width: 100%;
		margin-top: 0px;
		margin-bottom: 0px;
		border: 1px solid #666666;
		background: #FFFFFF;
	}
	#popTopicBookSelect .pop_foreground .pop_content a {
		text-decoration: none;
		font-size: 12pt;
		line-height: 1em;
		color: #000000;
		display: block;
	}
	#popTopicBookSelect .pop_foreground .pop_content a:hover {
		text-decoration: none;
		font-size: 12pt;
		line-height: 1em;
		color: #FFFFFF;
		background: #a69f00;
	}
	#popTopicBookSelect .pop_foreground  .pop_content p {
		margin-top: 0px;
		margin-bottom: 0px;
		font-size: 12pt;
		line-height: 1em;
	}
	#popTopicBookSelect .pop_foreground .pop_footer {
		background: #FFA100;
		text-align: center;
		font-size: 12pt;
		line-height: 1em;
		color: #993300;
	}

	#popTopicBookSelect #detailFormContainer {
	width: 322px;
	padding-top: 2px;
	padding-right: 10px;
	padding-bottom: 12px;
	padding-left: 10px;
	}
	#popTopicBookSelect #detailFormContainer br {
			clear: both;
	}
	#popTopicBookSelect #detailFormContainer #cascadingDropdowns div {
		  float: left;
		  margin-right: 10px;
	}
	#popTopicBookSelect #detailFormContainer label {
			float: left;
			margin-right: 10px;
			color: #FFFFFF;
			font: 24px &quot;標楷體&quot;;
	}
#popTopicBookSelect #confirm {
	text-align: right;
	margin-left: 225px;
	margin-top: 20px;
}
#popTopicBookSelect #confirm .button {
	background-color: #84A9D8;
	margin: 0px;
	border: 1px outset #CCC;
}
-->



		/inspireapp/
		
		
	    
	    	
	        
	          	Good Book Setup
	        
	        
      			
                    
                    
                          
	                          
	                            primary
	                            
	                          
	                          
	                            secondary
	                            
	                          
	                          
 		    						
							  
                          
                    
                    
                
            
	    

	
	

	 

			


		

		 

	

	

 
  
    Go To Page
  
  
 
 
  
 


 
		
	 






	
	var tm_timerID = window.setInterval(&quot;refreshTaskManager()&quot;, 10000);
	var init = 0;
	function refreshTaskManager() {
		var status = &quot; , &quot;'&quot; , &quot;S1&quot; , &quot;'&quot; , &quot;;
		var execute = 0;
		if (document.getElementById(&quot; , &quot;'&quot; , &quot;tmArea&quot; , &quot;'&quot; , &quot;) != null) {
			execute = 1;
			//status = document.getElementById(&quot; , &quot;'&quot; , &quot;taskStatus&quot; , &quot;'&quot; , &quot;).value;
		}
		if (execute == 1) {
			if ((status == &quot; , &quot;'&quot; , &quot;S1&quot; , &quot;'&quot; , &quot;) || (init == 0)) {
				document.getElementById(&quot; , &quot;'&quot; , &quot;taskManagerRefreshLink&quot; , &quot;'&quot; , &quot;).onclick();
			} else {
				clearInterval(tm_timerID);
			}
			init++;
		}
	}
	


 
  
  
     
  
 
 
  
 



	
	


 
  
    更改館藏狀態
  
  
 
 
  
 




		
	var tm_timerID = window.setInterval(&quot;refreshTaskManager()&quot;, 10000);
	var init = 0;
	function refreshTaskManager() {
		var status = &quot; , &quot;'&quot; , &quot;S1&quot; , &quot;'&quot; , &quot;;
		var execute = 0;
		if (document.getElementById(&quot; , &quot;'&quot; , &quot;tmArea&quot; , &quot;'&quot; , &quot;) != null) {
			execute = 1;
			//status = document.getElementById(&quot; , &quot;'&quot; , &quot;taskStatus&quot; , &quot;'&quot; , &quot;).value;
		}
		if (execute == 1) {
			if ((status == &quot; , &quot;'&quot; , &quot;S1&quot; , &quot;'&quot; , &quot;) || (init == 0)) {
				document.getElementById(&quot; , &quot;'&quot; , &quot;taskManagerRefreshLink&quot; , &quot;'&quot; , &quot;).onclick();
			} else {
				clearInterval(tm_timerID);
			}
			init++;
		}
	}
	


 
  
  
     
  
 
 
  
 




		
	

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
var field1 = new Ajax.Autocompleter(&quot;field1&quot;, &quot;field1choices&quot;, &quot;/inspireapp/internaltranzit/ManageInBatch,searchComponent.field1.sdirect?sp=Sfield1&amp;sp=S2&amp;sp=Sstarts+with&amp;sp=1&amp;updateParts=field1&quot;, {&quot;method&quot;:&quot;get&quot;,&quot;frequency&quot;:0.2,&quot;minChars&quot;:1,&quot;onFailure&quot;:tapestry.error,&quot;encoding&quot;:&quot;UTF-8&quot;});
var field2 = new Ajax.Autocompleter(&quot;field2&quot;, &quot;field2choices&quot;, &quot;/inspireapp/internaltranzit/ManageInBatch,searchComponent.field2.sdirect?sp=Sfield2&amp;sp=S30&amp;sp=Sstarts+with&amp;sp=2&amp;updateParts=field2&quot;, {&quot;method&quot;:&quot;get&quot;,&quot;frequency&quot;:0.2,&quot;minChars&quot;:1,&quot;onFailure&quot;:tapestry.error,&quot;encoding&quot;:&quot;UTF-8&quot;});
var field3 = new Ajax.Autocompleter(&quot;field3&quot;, &quot;field3choices&quot;, &quot;/inspireapp/internaltranzit/ManageInBatch,searchComponent.field3.sdirect?sp=Sfield3&amp;sp=S5&amp;sp=Sstarts+with&amp;sp=3&amp;updateParts=field3&quot;, {&quot;method&quot;:&quot;get&quot;,&quot;frequency&quot;:0.2,&quot;minChars&quot;:1,&quot;onFailure&quot;:tapestry.error,&quot;encoding&quot;:&quot;UTF-8&quot;});
var field4 = new Ajax.Autocompleter(&quot;field4&quot;, &quot;field4choices&quot;, &quot;/inspireapp/internaltranzit/ManageInBatch,searchComponent.field4.sdirect?sp=Sfield4&amp;sp=S7&amp;sp=Sstarts+with&amp;sp=4&amp;updateParts=field4&quot;, {&quot;method&quot;:&quot;get&quot;,&quot;frequency&quot;:0.2,&quot;minChars&quot;:1,&quot;onFailure&quot;:tapestry.error,&quot;encoding&quot;:&quot;UTF-8&quot;});
tapestry.cleanConnect(&quot;formSubmitSearch&quot;, &quot;onclick&quot;, &quot;event1167465096&quot;);
        tapestry.event1167465096=function(e){
            tapestry.event.stopEvent(e);
            
                
                    tapestry.form.submit(&quot;searchForm&quot;, &quot;formSubmitSearch&quot;, {&quot;async&quot;:true,&quot;json&quot;:false,&quot;url&quot;:&quot;/inspireapp/internaltranzit/ManageInBatch,searchComponent.searchForm.sdirect?updateParts=results&amp;updateParts=nrResults&amp;updateParts=autoRefreshZone&amp;updateParts=zSources&amp;updateParts=localSources&amp;updateParts=showOrderBy&amp;updateParts=noticeMessage&quot;});
                
                
            
            
        };
        tapestry.connect(&quot;formSubmitSearch&quot;, &quot;onclick&quot;, &quot;event1167465096&quot;);
tapestry.cleanConnect(&quot;browse&quot;, &quot;onclick&quot;, &quot;event1984482014&quot;);
        tapestry.event1984482014=function(e){
            tapestry.event.stopEvent(e);
            
                
                    tapestry.form.submit(&quot;searchForm&quot;, &quot;browse&quot;, {&quot;async&quot;:true,&quot;json&quot;:false,&quot;url&quot;:&quot;/inspireapp/internaltranzit/ManageInBatch,searchComponent.searchForm.sdirect?updateParts=results&amp;updateParts=nrResults&amp;updateParts=autoRefreshZone&amp;updateParts=inputField5&quot;});
                
                
            
            
        };
        tapestry.connect(&quot;browse&quot;, &quot;onclick&quot;, &quot;event1984482014&quot;);
tapestry.cleanConnect(&quot;Submit&quot;, &quot;onclick&quot;, &quot;event178957379&quot;);
        tapestry.event178957379=function(e){
            tapestry.event.stopEvent(e);
            
                
                    tapestry.form.submit(&quot;searchForm&quot;, &quot;Submit&quot;, {&quot;async&quot;:true,&quot;json&quot;:false,&quot;url&quot;:&quot;/inspireapp/internaltranzit/ManageInBatch,searchComponent.searchForm.sdirect?updateParts=results&amp;updateParts=nrResults&amp;updateParts=autoRefreshZone&amp;updateParts=inputField5&quot;});
                
                
            
            
        };
        tapestry.connect(&quot;Submit&quot;, &quot;onclick&quot;, &quot;event178957379&quot;);

tapestry.cleanConnect(&quot;sCriteria1&quot;, &quot;onchange&quot;, &quot;formEvent1172116752&quot;);
                tapestry.formEvent1172116752=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;internaltranzit/ManageInBatch/searchComponent.sCriteria1&quot;, bcomponentid:&quot;sCriteria1&quot;};
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
                tapestry.connect(&quot;sCriteria1&quot;, &quot;onchange&quot;, &quot;formEvent1172116752&quot;);
tapestry.cleanConnect(&quot;comparator1&quot;, &quot;onchange&quot;, &quot;formEvent308899038&quot;);
                tapestry.formEvent308899038=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;internaltranzit/ManageInBatch/searchComponent.comparator1&quot;, bcomponentid:&quot;comparator1&quot;};
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
                tapestry.connect(&quot;comparator1&quot;, &quot;onchange&quot;, &quot;formEvent308899038&quot;);
tapestry.cleanConnect(&quot;sCriteria2&quot;, &quot;onchange&quot;, &quot;formEvent1335983496&quot;);
                tapestry.formEvent1335983496=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;internaltranzit/ManageInBatch/searchComponent.sCriteria2&quot;, bcomponentid:&quot;sCriteria2&quot;};
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
                tapestry.connect(&quot;sCriteria2&quot;, &quot;onchange&quot;, &quot;formEvent1335983496&quot;);
tapestry.cleanConnect(&quot;comparator2&quot;, &quot;onchange&quot;, &quot;formEvent1479350283&quot;);
                tapestry.formEvent1479350283=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;internaltranzit/ManageInBatch/searchComponent.comparator2&quot;, bcomponentid:&quot;comparator2&quot;};
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
                tapestry.connect(&quot;comparator2&quot;, &quot;onchange&quot;, &quot;formEvent1479350283&quot;);
tapestry.cleanConnect(&quot;sCriteria3&quot;, &quot;onchange&quot;, &quot;formEvent886327579&quot;);
                tapestry.formEvent886327579=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;internaltranzit/ManageInBatch/searchComponent.sCriteria3&quot;, bcomponentid:&quot;sCriteria3&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;sCriteria3&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;sCriteria3&quot;, &quot;onchange&quot;, &quot;formEvent886327579&quot;);
tapestry.cleanConnect(&quot;comparator3&quot;, &quot;onchange&quot;, &quot;formEvent1930191548&quot;);
                tapestry.formEvent1930191548=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;internaltranzit/ManageInBatch/searchComponent.comparator3&quot;, bcomponentid:&quot;comparator3&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;comparator3&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;comparator3&quot;, &quot;onchange&quot;, &quot;formEvent1930191548&quot;);
tapestry.cleanConnect(&quot;sCriteria4&quot;, &quot;onchange&quot;, &quot;formEvent588250658&quot;);
                tapestry.formEvent588250658=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;internaltranzit/ManageInBatch/searchComponent.sCriteria4&quot;, bcomponentid:&quot;sCriteria4&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;sCriteria4&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;sCriteria4&quot;, &quot;onchange&quot;, &quot;formEvent588250658&quot;);
tapestry.cleanConnect(&quot;comparator4&quot;, &quot;onchange&quot;, &quot;formEvent851833462&quot;);
                tapestry.formEvent851833462=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;internaltranzit/ManageInBatch/searchComponent.comparator4&quot;, bcomponentid:&quot;comparator4&quot;};
                    tapestry.event.buildEventProperties(e, content, arguments);
                    if (!content[&quot;beventtarget.id&quot;]){
                    	content[&quot;beventtarget.id&quot;]=&quot;comparator4&quot;;
                    }
                    
                   var validateState=tapestry.form.forms[&quot;searchForm&quot;].validateForm;
                   var validateForm=false;
                   tapestry.form.setFormValidating(&quot;searchForm&quot;, validateForm);
                   
                    
                    tapestry.form.submitAsync(&quot;searchForm&quot;, content);
                    
                    
                    
                    tapestry.form.setFormValidating(&quot;searchForm&quot;, validateState);
                    
                };
                tapestry.connect(&quot;comparator4&quot;, &quot;onchange&quot;, &quot;formEvent851833462&quot;);
tapestry.cleanConnect(&quot;browseCriteria&quot;, &quot;onchange&quot;, &quot;formEvent1302783030&quot;);
                tapestry.formEvent1302783030=function(e){
                    var content={beventname:&quot;onchange&quot;, bcomponentidpath:&quot;internaltranzit/ManageInBatch/searchComponent.browseCriteria&quot;, bcomponentid:&quot;browseCriteria&quot;};
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
                tapestry.connect(&quot;browseCriteria&quot;, &quot;onchange&quot;, &quot;formEvent1302783030&quot;);
closeDialogComponent(&quot; , &quot;'&quot; , &quot;TinreadDialog&quot; , &quot;'&quot; , &quot;);

closeDialogComponent(&quot; , &quot;'&quot; , &quot;GlobalChangeStatus&quot; , &quot;'&quot; , &quot;);
try {
  attachFocus(&quot; , &quot;'&quot; , &quot;field1&quot; , &quot;'&quot; , &quot;);
 }
 catch(e) {}
closeDialogComponent(&quot; , &quot;'&quot; , &quot;IGCD&quot; , &quot;'&quot; , &quot;);
closeDialogComponent(&quot; , &quot;'&quot; , &quot;GlobalChangeStatus_1&quot; , &quot;'&quot; , &quot;);
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



            
                
                    確定
                    取消
                
                
                    
                        
                            CMUL - 神資圖書館
                            
                                
                            
                        
                    
                    
                    
                        
                            2 - 2
                            
                                
                            
                        
                    
                    
                
                    
                        
                            123 - 123
                            
                                
                            
                        
                    
                    
                
                    
                        
                            20230417 - 20230417
                            
                                
                            
                        
                    
                    
                
                    
                        
                            20230418 - 20230418
                            
                                
                            
                        
                    
                    
                
                    
                        
                            AH - 安南醫院
                            
                                
                            
                        
                    
                    
                    
                        
                            AHGL - 安南圖書區
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            av - av
                            
                                
                            
                        
                    
                    
                
                    
                        
                            B007 - B007
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BCSB4 - BCSB4
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BX - 取書櫃1
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BY - 取書櫃2
                            
                                
                            
                        
                    
                    
                
                    
                        
                            CB - 北港分館
                            
                                
                            
                        
                    
                    
                    
                        
                            BAVN - 北港分館視聽區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BCIR - 北港分館流通櫃檯
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BCRA - 北港分館指參(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BPAV - 北港分館視聽區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BPCL - 北港分館書庫
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            cbook - cbook
                            
                                
                            
                        
                    
                    
                
                    
                        
                            circd - circd
                            
                                
                            
                        
                    
                    
                
                    
                        
                            clp - clp
                            
                                
                            
                        
                    
                    
                
                    
                        
                            CM - 北港附設醫院
                            
                                
                            
                        
                    
                    
                    
                        
                            BMHL - 北港附設醫院圖書室
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            CMUL - 中國醫藥大學圖書館 - CMUL - 中國醫藥大學圖書館
                            
                                
                            
                        
                    
                    
                
                    
                        
                            CU - 台中總館
                            
                                
                            
                        
                    
                    
                    
                        
                            CUAV - 台中總館視聽區(獨立專區)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MAVN - 台中總館視聽區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MAVR - 台中總館視聽區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCAT - 台中總館技服組
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCBS - 台中總館密閉書庫
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCIR - 台中總館流通櫃檯
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCRA - 台中總館教師指定參考書(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MCSS - 台中總館B3裝訂期刊區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MDIA - 台中總館博碩士論文區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MEAS - 台中總館探索史懷哲之路專書區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MEXM - 台中總館國考書區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MFGA - 台中總館本校教職優良教材區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MFPA - 台中總館本校教師升等資料區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MFSA - 台中總館本校教職論著
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MHME - 台中總館人文專書區-醫學倫理(限館內閱覽)台中總館人文專書區-醫學倫理(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MHMH - 台中總館人文專書區-醫療史(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MHML - 台中總館人文專書區-醫學法律(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MHQC - 台中總館人文專書區-醫療品質(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MNBR - 台中總館新書展示區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MPAA - 台中總館績效暨獲獎區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MPBL - 台中總館PBL專書區(限館內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MREF - 台中總館參考室
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MRSS - 台中總館閱覽組
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MS-C - 台中總館期刊複本櫃
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSER - 台中總館期刊區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSPA - 台中總館研究計劃專書
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSPB - 中醫醫史文獻室(限所內閱覽)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSPC - 台中總館特藏室
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MSTK - 台中總館書庫
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MYBK - 台中總館參考壁櫃
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 7 - new item 7
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ONLN - 台中總館線上資料
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            e-resources - 電子資源
                            
                                
                            
                        
                    
                    
                
                    
                        
                            EB-P - EB-P
                            
                                
                            
                        
                    
                    
                
                    
                        
                            elect - elect
                            
                                
                            
                        
                    
                    
                
                    
                        
                            H-EQ - H-EQ
                            
                                
                            
                        
                    
                    
                
                    
                        
                            H-MR - H-MR
                            
                                
                            
                        
                    
                    
                
                    
                        
                            L - L
                            
                                
                            
                        
                    
                    
                
                    
                        
                            L40 - L40
                            
                                
                            
                        
                    
                    
                
                    
                        
                            LB 圖書總館 - LB 圖書總館
                            
                                
                            
                        
                    
                    
                
                    
                        
                            LB-S - LB-S
                            
                                
                            
                        
                    
                    
                
                    
                        
                            LE - 語文教學中心
                            
                                
                            
                        
                    
                    
                    
                        
                            LEGL - 語文教學中心圖書室
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            LIB - LIB
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 1 - new item 1
                            
                                
                            
                        
                    
                    
                    
                        
                            new item 3 - new item 3
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            new item 10 - new item 10
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 12 - new item 12
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 13 - new item 13
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 14 - new item 14
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 16 - new item 16
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 2 - new item 2
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 20 - new item 20
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 4 - new item 4
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 5 - new item 5
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 6 - 英才校區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 8 - new item 8
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 9 - new item 9
                            
                                
                            
                        
                    
                    
                
                    
                        
                            NPTU - NPTU
                            
                                
                            
                        
                    
                    
                
                    
                        
                            OUK - OUK
                            
                                
                            
                        
                    
                    
                
                    
                        
                            PT - 培德醫院
                            
                                
                            
                        
                    
                    
                    
                        
                            new item 11 - new item 11
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 17 - new item 17
                            
                                
                            
                        
                    
                    
                
                    
                        
                            PTGL - 培德醫院圖書區
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            ptext - ptext
                            
                                
                            
                        
                    
                    
                
                    
                        
                            SB3 - SB3
                            
                                
                            
                        
                    
                    
                
                    
                        
                            T-P - T-P
                            
                                
                            
                        
                    
                    
                
                    
                        
                            TBBK - TBBK
                            
                                
                            
                        
                    
                    
                
                    
                        
                            TH - 台北分院
                            
                                
                            
                        
                    
                    
                    
                        
                            THGL - 台北分院圖書區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            THPA - 台北分院期刊區
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            W-P - W-P
                            
                                
                            
                        
                    
                    
                
                    
                        
                            YH - 豐原分院
                            
                                
                            
                        
                    
                    
                    
                        
                            new item 18 - new item 18
                            
                                
                            
                        
                    
                    
                
                    
                        
                            new item 19 - new item 19
                            
                                
                            
                        
                    
                    
                
                    
                        
                            YHGL - 豐原分院圖書區
                            
                                
                            
                        
                    
                    
                
                
                    
                        
                            z3llc - z3llc
                            
                                
                            
                        
                    
                    
                
                    
                        
                            z6bkf - z6bkf
                            
                                
                            
                        
                    
                    
                
                    
                        
                            zd1a2 - zd1a2
                            
                                
                            
                        
                    
                    
                
                    
                        
                            zd1e - zd1e
                            
                                
                            
                        
                    
                    
                
                    
                        
                            zdlf - zdlf
                            
                                
                            
                        
                    
                    
                
                    
                        
                            五樓漫畫書專區 - 五樓漫畫書專區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            實體館藏 - 實體館藏
                            
                                
                            
                        
                    
                    
                
                    
                        
                            綜合書庫 - 綜合書庫
                            
                                
                            
                        
                    
                    
                
                    
                        
                            艾迪訊圖書館 - 艾迪訊圖書館
                            
                                
                            
                        
                    
                    
                
                    
                        
                            附中出版物專區 - 附中出版物專區
                            
                                
                            
                        
                    
                    
                
                    
                        
                            龍華科技大學圖書館 - 龍華科技大學圖書館
                            
                                
                            
                        
                    
                    
                
                
            
                
                    確定
                    取消
                
                
                    
                        
                            B可借圖書
                            
                                
                            
                        
                    
                    
                
                    
                        
                            B電子資源
                            
                                
                            
                        
                    
                    
                
                    
                        
                            eee
                            
                                
                            
                        
                    
                    
                
                    
                        
                            M可借行動設備
                            
                                
                            
                        
                    
                    
                
                    
                        
                            P可借期刊
                            
                                
                            
                        
                    
                    
                
                    
                        
                            V可借視聽
                            
                                
                            
                        
                    
                    
                
                    
                        
                            www
                            
                                
                            
                        
                    
                    
                
                    
                        
                            不流通
                            
                                
                            
                        
                    
                    
                
                    
                        
                            書箱借閱30天
                            
                                
                            
                        
                    
                    
                
            
                
                    確定
                    取消
                
                
                    
                        
                            BD - 藍光光碟
                            
                                
                            
                        
                    
                    
                
                    
                        
                            CA - 靜畫資料
                            
                                
                            
                        
                    
                    
                
                    
                        
                            DB - 資料庫
                            
                                
                            
                        
                    
                    
                
                    
                        
                            DF - 磁片
                            
                                
                            
                        
                    
                    
                
                    
                        
                            DO - 電子書
                            
                                
                            
                        
                    
                    
                
                    
                        
                            EA - 立體模型
                            
                                
                            
                        
                    
                    
                
                    
                        
                            EB - 線上電子書
                            
                                
                            
                        
                    
                    
                
                    
                        
                            EJ - 線上電子期刊
                            
                                
                            
                        
                    
                    
                
                    
                        
                            EP - 電子期刊光碟
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ERROR - 有問題特藏
                            
                                
                            
                        
                    
                    
                
                    
                        
                            FA - 磁帶
                            
                                
                            
                        
                    
                    
                
                    
                        
                            KT - 多媒體組件
                            
                                
                            
                        
                    
                    
                
                    
                        
                            LA - 地圖
                            
                                
                            
                        
                    
                    
                
                    
                        
                            LD - 影碟
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MP - MP3
                            
                                
                            
                        
                    
                    
                
                    
                        
                            NH - 微縮單片
                            
                                
                            
                        
                    
                    
                
                    
                        
                            NR - 微縮捲片
                            
                                
                            
                        
                    
                    
                
                    
                        
                            QA - 地球儀
                            
                                
                            
                        
                    
                    
                
                    
                        
                            R - 參考書
                            
                                
                            
                        
                    
                    
                
                    
                        
                            SL - 幻燈片
                            
                                
                            
                        
                    
                    
                
                    
                        
                            VC - 錄影帶
                            
                                
                            
                        
                    
                    
                
                    
                        
                            VD - VCD
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BOX - 書箱
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ERM_DB - 電子資料庫(ERM)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ERM_WS - 網路資源(ERM)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ERM_EB - 電子書(ERM)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ERM_EJ - 電子期刊(ERM)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            XL - X-ray
                            
                                
                            
                        
                    
                    
                
                    
                        
                            BOOK - 圖書
                            
                                
                            
                        
                    
                    
                
                    
                        
                            AC - 錄音帶
                            
                                
                            
                        
                    
                    
                
                    
                        
                            APP - 附件
                            
                                
                            
                        
                    
                    
                
                    
                        
                            P - 現期期刊
                            
                                
                            
                        
                    
                    
                
                    
                        
                            ac - ac
                            
                                
                            
                        
                    
                    
                
                    
                        
                            DD - 影像光碟(DVD)
                            
                                
                            
                        
                    
                    
                
                    
                        
                            MD - 行動設備
                            
                                
                            
                        
                    
                    
                
                    
                        
                            S - 裝訂期刊
                            
                                
                            
                        
                    
                    
                
                    
                        
                            AD - 唱片
                            
                                
                            
                        
                    
                    
                
                    
                        
                            CD - 光碟
                            
                                
                            
                        
                    
                    
                
                    
                        
                            booklet - 小冊子
                            
                                
                            
                        
                    
                    
                
                    
                        
                            KKtest - KK
                            
                                
                            
                        
                    
                    
                
                    
                        
                            YYtest2 - YY2
                            
                                
                            
                        
                    
                    
                
                    
                        
                            YYtest4 - YYYtest
                            
                                
                            
                        
                    
                    
                
                    
                        
                            0425 - 0425
                            
                                
                            
                        
                    
                    
                
                    
                        
                            TEST - TEST
                            
                                
                            
                        
                    
                    
                
                    
                        
                            TEST0425 - TEST0425
                            
                                
                            
                        
                    
                    
                id(&quot;field2&quot;)&quot;))]</value>
      <webElementGuid>20ef992c-043b-4386-889b-7c58fb6d8bfa</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
