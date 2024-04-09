<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_dojo.registerModulePath(tapestry, insp_118d36</name>
   <tag></tag>
   <elementGuidId>fa325989-2191-46ce-b05d-3ef0e856dd3d</elementGuidId>
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
      <webElementGuid>f0739a94-83fd-45dd-b443-51962387b1d2</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>leftmargin</name>
      <type>Main</type>
      <value>0</value>
      <webElementGuid>9f9e968d-6269-4554-a307-2a732f8b5eb6</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>topmargin</name>
      <type>Main</type>
      <value>0</value>
      <webElementGuid>8b516eb1-695e-4adc-8bf4-4331e5b2ac25</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>rightmargin</name>
      <type>Main</type>
      <value>0</value>
      <webElementGuid>c50e3801-197c-4023-b1dd-ec8a9b4c2d59</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>bottommargin</name>
      <type>Main</type>
      <value>0</value>
      <webElementGuid>3220c1c0-04c2-4202-820a-4009be88cd42</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>bgcolor</name>
      <type>Main</type>
      <value>#ffffff</value>
      <webElementGuid>302e2e6c-6d31-4844-bed5-65991f3ee1b9</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>Body</value>
      <webElementGuid>f51af6f5-08e9-4daf-8b8e-4134a78b0093</webElementGuid>
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

dojo.require(&quot;tapestry.fx&quot;);
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
// -->


	
	
	







		
	
	
		
			
				
				Hi, catc (CU)
				
				最近登入:2024-03-13 13:30:02 ,成功
				
				
					
					
                       0 筆新訊息
                    
                
				
				繁體中文English繁體中文
				
English
繁體中文
						
						
				
				
					
					登 出
				
			
		
		
		
			
			
				神資圖書館(15trunk機)
			
			
		
	
	
	
	
	
		
		
		
			
			
			 查詢  編目  流通  採購   期刊控制   場地設備  說明  我的帳戶  學科服務  管理  統計  清單  Portal  臉辨報表  ERM 
			簡單查詢進階查詢FRBR查詢最近的查詢已儲存的記錄FRBR作品書目書目記錄維護註記刪除之書目記錄書目記錄全域修改權威記錄權威記錄維護註記刪除之權威記錄權威記錄全域修改條碼號館藏資料維護註記刪除之館藏資料維護出版者出版者資料維護內部移送作業書單維護FRBR作品辦證讀者記錄維護檢查讀者註記刪除之讀者記錄讀者證卡記錄維護離職離校借閱檯讀者流通檯還書箱讀者查詢交易紀錄違規讀者查詢讀者個人違規記錄指定參考書交易歷史查詢一般搜尋交易館藏預約順位調整題名預約順位調整批次修改到期日內部移送作業盤點盤點維護跨館預約/調閱跨館預約/調閱需求移轉-寄送項目移轉-收到項目盤點比對批次報廢選擇報廢執行報廢已報廢歷史查詢書箱作業條碼號空號清單(條碼機印製)訂購檢查薦購請購訂購明細訂單登收用卷期登收程序發票款項維護預算經費供應商標案內部移送作業催缺候選催缺項目催缺記錄期刊主檔訂購檢查薦購請購核可訂購明細訂單登收用卷期登收程序發票款項維護預算經費供應商裝訂裝訂主檔維護擬裝設定送裝確認裝訂登收標案內部移送作業催缺候選催缺項目催缺記錄流通記錄預約記錄借用記錄維護分類維護單項維護參數設定流通政策維護預約審核場地設備黑名單書籤訊息排程工作日誌學科館藏館藏資料類型設定學科館藏維護學科預約教師資料維護讀者評價選單學科預約維護學科預約查詢教師評語學科報表學科館藏冊數統計教師學科服務時數與使用人次統計學科服務讀者清單學科報表學科館藏冊數統計教師學科服務時數與使用人次統計學科服務讀者清單一般設定類型/使用者(館員)片語設定系統設定貨幣匯率館藏地範本設定--依館員帳號翻譯轉換 選取清單格式屬性條碼號規範顯示設定圖書館系統屬性推薦範本設定館員新增申請QR Code 設定編目書目類型設定MARC規範格式設定顯示設定停用字SDI維護同義字條碼號原則對應流通流通通知單範本流通通知單參數設定流通通知單發送紀錄日常作業排程一般每周開閉館設定閉館日期維護交易紀錄類型交易方式可適用讀者身份交易方式可適用館藏流通類別交易方式可適用使用類別交易紀錄-館藏狀態流通政策讀者證卡原則單位所系夾書單欄位設定讀者閱讀興趣發送紀錄期刊週期傳閱清單維護 轉入Z39屬性設定更新書封上傳書封索引編修索引重建功能維護公佈欄維護意見回覆維護主題維護特別書籍維護心得分享東區互惠審核東區互惠匯出系統代碼維護貨幣代碼作者號維護科特號維護OPACSDI維護讀者評論管理Help維護熱門關鍵字過濾館員Log記錄館員登入/出記錄館員資料變更記錄讀者資料變更記錄權限設定MARC權限設定功能權限設定角色設定系統狀態排程管理排程監控者排程日誌查詢統計檢索點/檢索語統計報表不同時段使用次數統計書目下載次數統計電子資源統計電子資源-館藏點閱次數統計電子資源-讀者點閱次數統計電子資源-單位系所點閱次數統計編目統計館藏量統計報表類號區間統計報表圖書增減統計報表編目/館別/大類交叉分析報表大類/冊數統計表大類/特藏交叉統計表特藏/冊數統計表新書特藏大類交叉統計圖書增減表／月份／大類大類語言種數冊數統計大類/條碼號類別冊數統計圖書冊數分類統計單圖書冊數金額分類統計非書資料數量金額統計資料類型範圍館藏統計表編目工作量統計(個人)編目工作量統計(各館)館藏狀態統計報表流通統計讀者借閱排行榜書展借閱清單流通人數統計(學制分頁 符合科系)讀者類型借閱暨歸還流通量統計報表流通人次冊數統計  利用概況統計(日報表)讀者借閱交叉分析各館特藏預約統計報表館內使用統計讀者身份借閱統計讀者個人及單位所系借閱排行榜館藏資料借閱統計單位系所借閱排行榜單位系所借閱統計熱門借閱排行榜熱門預約排行榜讀者辦証暨補發人次統計表借閱交叉分析統計資料類型流通統計館員流通量統計單位(個人)保管圖書(非書)數量明細館際互借代還統計-本館館藏館際互借代還統計-他館館藏指定參考書借閱量統計表各大類熱門借閱排行榜大專院校圖書館讀者身份借閱清單及統計報表館藏流通量統計跨館預約/調閱館藏借閱排行榜罰款收款人數/次數統計跨館預約/調閱使用量統計罰款收款統計特定館藏借閱統計採購統計書商採購統計年度經費採購統計特藏訂購方式金額統計書商到書率書商績效統計期刊統計期刊到刊率統計報表平均收刊日統計報表歷年期刊借閱統計書商到書率書商催缺次數書商累計訂購次數期刊訂費漲幅統計逾期違約金統計期刊經費訂購統計報表場地設備場地設備使用統計表報表模組範本維護編目清單書目標籤報表Tag856檢查報表新版書標維護次分類交叉分析登錄簿條碼產生清單登錄簿清單條碼號空號清單單位科系館藏設定清單館藏狀態預約清單特殊館藏清單館藏狀態清單報表流通清單罰款未收款報表預約待取清單借閱(逾期)清單還書清單新書清單讀者欠款清單借閱逾期清單罰款收款清單預約撤架清單長期借閱圖書清單離線流通清單預約額滿資料清單單位系所借閱逾期清單跨館還書清單離職離校清單跨館移送清單預約保留結束清單指定參考書借閱量明細表未外借館藏清單重複讀者比對報表借閱逾期通知單取書櫃上架清單預約圖書清單採購清單年度經費採買書籍運用率訂購資料明細表擬購資料明細表請購單詢價單訂購逾期清單謝函書的催缺報表薦購書刊複本比對報表採購參考清單財產增加清冊財產減少清冊已付款未到清單訂購明細期刊清單期刊合訂本登錄簿交贈期刊清單期刊架位清單裝訂歷史清單薦購書刊複本比對報表期刊登錄簿索贈清單期刊查驗清單已付款未到清單場地設備預約取消清單臉辨用戶使用分析表臉辨用戶與圖書分類關係表期間辦證統計_年報期間辦證統計_月報期間辦證明細期間辦證異動統計_年報期間辦證異動統計_月報辦證異動明細臉辨使用紀錄統計_年報臉辨使用紀錄統計_月報臉辨使用紀錄明細人數統計表人數統計明細表人次統計表人次統計明細表熱門時段統計表讀者進出排行榜期間辦證刪除明細系統參數設定IP區間管理延伸查詢設定系統設定檔資源瀏覽/後分類設定資源到期通知設定可使用身分類別設定 代碼管理所屬資料庫清單共用代碼檔共用代碼類別電子資源管理資源前端顯示設定Ezproxy設定檔維護電子資料庫/網路資源電子期刊/電子書電子資源批次匯入電子資源副檔匯入讀者回報資源連線異常 Meniuri.erm.9000報表點閱資源排行榜依學院別使用統計依單位別使用統計依月分資源使用統計依學院登入統計表依系所登入統計表依身分登入統計表登入清單依身分別使用統計表使用者使用資源清單 收藏資源排行榜偵測電子資源狀態設定偵測資源時間設定偵測電子資源查詢電子資源狀態電子資源狀況報表
				
	


		
			
				流通 > 盤點 > 盤點維護		
			

 
  建立範本    

     
   


	
			
				23 筆 •
				頁數 3 •
			
					
			
			
				
		       
			
			
			     
			
				
		    
			
				
				
		          
			      1
		         
				
			
				
				
		          
			      2
		         
				
			
				
					3  	
				
				
			
			
		     
			 
		     
			 	
			     
			 
			
	


	
     
	     
	      
	      
	       序號
	       功能圖示
	       盤點名稱
	       盤點狀態
	       備註
	       母條件搜尋語法
	      
	      
	      
	        21
	        
	          
	          
	          
	          
	        
	        20230424
	        啟動盤點
	        
	        條碼號 starts with 'test' order by 條碼號
	      
	        22
	        
	          
	            
	          
	          
	          
	            
	          
	          
	        
	        2023
	        尚未啟動盤點
	        
	        條碼號 starts with '' order by 條碼號
	      
	        23
	        
	          
	          
	          
	          
	        
	        測試盤點_20240313
	        啟動盤點
	        測試盤點_202403130145
	         (條碼號 starts with '0000000001') and 條碼號 &lt;= '0000000018' order by 條碼號
	      
	     
   

	
			
				23 筆 •
				頁數 3 •
			
					
			
			
				
		       
			
			 
				
		    
			
				
				
		          
			      1
		         
				
			
				
				
		          
			      2
		         
				
			
				
					3  	
				
				
			
			
		     
			  
		     
			 	
			     
			 
				
		


   

   
   
   
     
     
      
       
         
          
           盤點母樣本數:
           2
           
          
          
           盤點上傳數:
           1
           清除資料
          
          
           前次作業:
           ACTIVATE
           
          
          
         
         條碼號
             '0000000001
         
		 
		   館藏狀態
		     
		 
		 
		   比對結果
		     不存在資料庫
		 
       
       
        
        
	         






	            
	            
	             刷入條碼號
	             
	             
	             輸入超過255字元
	            
	         
         
         








            
            

            
             上傳條碼號文字檔(.txt)

            
            
            
            
            上傳盤點檔案
            
            
            
            
         

        
       
      
     
     
	     
	      
	            使失效
	      
	     
	       檢查
	     
	     
	      執行
	     
	     
	     
	     取消
   
  
 

 

			
		
	




		
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

tapestry.form.registerForm(&quot;OnShelfForm&quot;);
tapestry.cleanConnect(&quot;btnUpload&quot;, &quot;onclick&quot;, &quot;event592121007&quot;);
        tapestry.event592121007=function(e){
            tapestry.event.stopEvent(e);
            
                
                    tapestry.form.submit(&quot;OnShelfForm&quot;, &quot;btnUpload&quot;, {&quot;async&quot;:true,&quot;json&quot;:false,&quot;url&quot;:&quot;/inspireapp/circulation/inventory/InventoryMaintain,OnShelfForm.sdirect?updateParts=InventoryDetails&amp;updateParts=nrResults&quot;});
                
                
            
            
        };
        tapestry.connect(&quot;btnUpload&quot;, &quot;onclick&quot;, &quot;event592121007&quot;);

tapestry.form.registerForm(&quot;Form&quot;);

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
closeDialogComponent('TinreadMessageDialog');

tapestry.form.focusField('inputTxtField');});
// -->



id(&quot;For_5&quot;)/td[6]</value>
      <webElementGuid>42b5dc9c-ac4e-4c3c-acbd-2c5f59023ef3</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;Body&quot;)</value>
      <webElementGuid>89c56e29-4c8b-4906-9fc5-6a963632e985</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//body[@id='Body']</value>
      <webElementGuid>8874c7ec-46b0-4551-a04b-ad998c4c9e40</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>14ea1c03-be40-44ce-a5f4-0623fce48910</webElementGuid>
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

dojo.require(&quot;tapestry.fx&quot;);
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
// -->


	
	
	







		
	
	
		
			
				
				Hi, catc (CU)
				
				最近登入:2024-03-13 13:30:02 ,成功
				
				
					
					
                       0 筆新訊息
                    
                
				
				繁體中文English繁體中文
				
English
繁體中文
						
						
				
				
					
					登 出
				
			
		
		
		
			
			
				神資圖書館(15trunk機)
			
			
		
	
	
	
	
	
		
		
		
			
			
			 查詢  編目  流通  採購   期刊控制   場地設備  說明  我的帳戶  學科服務  管理  統計  清單  Portal  臉辨報表  ERM 
			簡單查詢進階查詢FRBR查詢最近的查詢已儲存的記錄FRBR作品書目書目記錄維護註記刪除之書目記錄書目記錄全域修改權威記錄權威記錄維護註記刪除之權威記錄權威記錄全域修改條碼號館藏資料維護註記刪除之館藏資料維護出版者出版者資料維護內部移送作業書單維護FRBR作品辦證讀者記錄維護檢查讀者註記刪除之讀者記錄讀者證卡記錄維護離職離校借閱檯讀者流通檯還書箱讀者查詢交易紀錄違規讀者查詢讀者個人違規記錄指定參考書交易歷史查詢一般搜尋交易館藏預約順位調整題名預約順位調整批次修改到期日內部移送作業盤點盤點維護跨館預約/調閱跨館預約/調閱需求移轉-寄送項目移轉-收到項目盤點比對批次報廢選擇報廢執行報廢已報廢歷史查詢書箱作業條碼號空號清單(條碼機印製)訂購檢查薦購請購訂購明細訂單登收用卷期登收程序發票款項維護預算經費供應商標案內部移送作業催缺候選催缺項目催缺記錄期刊主檔訂購檢查薦購請購核可訂購明細訂單登收用卷期登收程序發票款項維護預算經費供應商裝訂裝訂主檔維護擬裝設定送裝確認裝訂登收標案內部移送作業催缺候選催缺項目催缺記錄流通記錄預約記錄借用記錄維護分類維護單項維護參數設定流通政策維護預約審核場地設備黑名單書籤訊息排程工作日誌學科館藏館藏資料類型設定學科館藏維護學科預約教師資料維護讀者評價選單學科預約維護學科預約查詢教師評語學科報表學科館藏冊數統計教師學科服務時數與使用人次統計學科服務讀者清單學科報表學科館藏冊數統計教師學科服務時數與使用人次統計學科服務讀者清單一般設定類型/使用者(館員)片語設定系統設定貨幣匯率館藏地範本設定--依館員帳號翻譯轉換 選取清單格式屬性條碼號規範顯示設定圖書館系統屬性推薦範本設定館員新增申請QR Code 設定編目書目類型設定MARC規範格式設定顯示設定停用字SDI維護同義字條碼號原則對應流通流通通知單範本流通通知單參數設定流通通知單發送紀錄日常作業排程一般每周開閉館設定閉館日期維護交易紀錄類型交易方式可適用讀者身份交易方式可適用館藏流通類別交易方式可適用使用類別交易紀錄-館藏狀態流通政策讀者證卡原則單位所系夾書單欄位設定讀者閱讀興趣發送紀錄期刊週期傳閱清單維護 轉入Z39屬性設定更新書封上傳書封索引編修索引重建功能維護公佈欄維護意見回覆維護主題維護特別書籍維護心得分享東區互惠審核東區互惠匯出系統代碼維護貨幣代碼作者號維護科特號維護OPACSDI維護讀者評論管理Help維護熱門關鍵字過濾館員Log記錄館員登入/出記錄館員資料變更記錄讀者資料變更記錄權限設定MARC權限設定功能權限設定角色設定系統狀態排程管理排程監控者排程日誌查詢統計檢索點/檢索語統計報表不同時段使用次數統計書目下載次數統計電子資源統計電子資源-館藏點閱次數統計電子資源-讀者點閱次數統計電子資源-單位系所點閱次數統計編目統計館藏量統計報表類號區間統計報表圖書增減統計報表編目/館別/大類交叉分析報表大類/冊數統計表大類/特藏交叉統計表特藏/冊數統計表新書特藏大類交叉統計圖書增減表／月份／大類大類語言種數冊數統計大類/條碼號類別冊數統計圖書冊數分類統計單圖書冊數金額分類統計非書資料數量金額統計資料類型範圍館藏統計表編目工作量統計(個人)編目工作量統計(各館)館藏狀態統計報表流通統計讀者借閱排行榜書展借閱清單流通人數統計(學制分頁 符合科系)讀者類型借閱暨歸還流通量統計報表流通人次冊數統計  利用概況統計(日報表)讀者借閱交叉分析各館特藏預約統計報表館內使用統計讀者身份借閱統計讀者個人及單位所系借閱排行榜館藏資料借閱統計單位系所借閱排行榜單位系所借閱統計熱門借閱排行榜熱門預約排行榜讀者辦証暨補發人次統計表借閱交叉分析統計資料類型流通統計館員流通量統計單位(個人)保管圖書(非書)數量明細館際互借代還統計-本館館藏館際互借代還統計-他館館藏指定參考書借閱量統計表各大類熱門借閱排行榜大專院校圖書館讀者身份借閱清單及統計報表館藏流通量統計跨館預約/調閱館藏借閱排行榜罰款收款人數/次數統計跨館預約/調閱使用量統計罰款收款統計特定館藏借閱統計採購統計書商採購統計年度經費採購統計特藏訂購方式金額統計書商到書率書商績效統計期刊統計期刊到刊率統計報表平均收刊日統計報表歷年期刊借閱統計書商到書率書商催缺次數書商累計訂購次數期刊訂費漲幅統計逾期違約金統計期刊經費訂購統計報表場地設備場地設備使用統計表報表模組範本維護編目清單書目標籤報表Tag856檢查報表新版書標維護次分類交叉分析登錄簿條碼產生清單登錄簿清單條碼號空號清單單位科系館藏設定清單館藏狀態預約清單特殊館藏清單館藏狀態清單報表流通清單罰款未收款報表預約待取清單借閱(逾期)清單還書清單新書清單讀者欠款清單借閱逾期清單罰款收款清單預約撤架清單長期借閱圖書清單離線流通清單預約額滿資料清單單位系所借閱逾期清單跨館還書清單離職離校清單跨館移送清單預約保留結束清單指定參考書借閱量明細表未外借館藏清單重複讀者比對報表借閱逾期通知單取書櫃上架清單預約圖書清單採購清單年度經費採買書籍運用率訂購資料明細表擬購資料明細表請購單詢價單訂購逾期清單謝函書的催缺報表薦購書刊複本比對報表採購參考清單財產增加清冊財產減少清冊已付款未到清單訂購明細期刊清單期刊合訂本登錄簿交贈期刊清單期刊架位清單裝訂歷史清單薦購書刊複本比對報表期刊登錄簿索贈清單期刊查驗清單已付款未到清單場地設備預約取消清單臉辨用戶使用分析表臉辨用戶與圖書分類關係表期間辦證統計_年報期間辦證統計_月報期間辦證明細期間辦證異動統計_年報期間辦證異動統計_月報辦證異動明細臉辨使用紀錄統計_年報臉辨使用紀錄統計_月報臉辨使用紀錄明細人數統計表人數統計明細表人次統計表人次統計明細表熱門時段統計表讀者進出排行榜期間辦證刪除明細系統參數設定IP區間管理延伸查詢設定系統設定檔資源瀏覽/後分類設定資源到期通知設定可使用身分類別設定 代碼管理所屬資料庫清單共用代碼檔共用代碼類別電子資源管理資源前端顯示設定Ezproxy設定檔維護電子資料庫/網路資源電子期刊/電子書電子資源批次匯入電子資源副檔匯入讀者回報資源連線異常 Meniuri.erm.9000報表點閱資源排行榜依學院別使用統計依單位別使用統計依月分資源使用統計依學院登入統計表依系所登入統計表依身分登入統計表登入清單依身分別使用統計表使用者使用資源清單 收藏資源排行榜偵測電子資源狀態設定偵測資源時間設定偵測電子資源查詢電子資源狀態電子資源狀況報表
				
	


		
			
				流通 > 盤點 > 盤點維護		
			

 
  建立範本    

     
   


	
			
				23 筆 •
				頁數 3 •
			
					
			
			
				
		       
			
			
			     
			
				
		    
			
				
				
		          
			      1
		         
				
			
				
				
		          
			      2
		         
				
			
				
					3  	
				
				
			
			
		     
			 
		     
			 	
			     
			 
			
	


	
     
	     
	      
	      
	       序號
	       功能圖示
	       盤點名稱
	       盤點狀態
	       備註
	       母條件搜尋語法
	      
	      
	      
	        21
	        
	          
	          
	          
	          
	        
	        20230424
	        啟動盤點
	        
	        條碼號 starts with &quot; , &quot;'&quot; , &quot;test&quot; , &quot;'&quot; , &quot; order by 條碼號
	      
	        22
	        
	          
	            
	          
	          
	          
	            
	          
	          
	        
	        2023
	        尚未啟動盤點
	        
	        條碼號 starts with &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; order by 條碼號
	      
	        23
	        
	          
	          
	          
	          
	        
	        測試盤點_20240313
	        啟動盤點
	        測試盤點_202403130145
	         (條碼號 starts with &quot; , &quot;'&quot; , &quot;0000000001&quot; , &quot;'&quot; , &quot;) and 條碼號 &lt;= &quot; , &quot;'&quot; , &quot;0000000018&quot; , &quot;'&quot; , &quot; order by 條碼號
	      
	     
   

	
			
				23 筆 •
				頁數 3 •
			
					
			
			
				
		       
			
			 
				
		    
			
				
				
		          
			      1
		         
				
			
				
				
		          
			      2
		         
				
			
				
					3  	
				
				
			
			
		     
			  
		     
			 	
			     
			 
				
		


   

   
   
   
     
     
      
       
         
          
           盤點母樣本數:
           2
           
          
          
           盤點上傳數:
           1
           清除資料
          
          
           前次作業:
           ACTIVATE
           
          
          
         
         條碼號
             &quot; , &quot;'&quot; , &quot;0000000001
         
		 
		   館藏狀態
		     
		 
		 
		   比對結果
		     不存在資料庫
		 
       
       
        
        
	         






	            
	            
	             刷入條碼號
	             
	             
	             輸入超過255字元
	            
	         
         
         








            
            

            
             上傳條碼號文字檔(.txt)

            
            
            
            
            上傳盤點檔案
            
            
            
            
         

        
       
      
     
     
	     
	      
	            使失效
	      
	     
	       檢查
	     
	     
	      執行
	     
	     
	     
	     取消
   
  
 

 

			
		
	




		
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

tapestry.form.registerForm(&quot;OnShelfForm&quot;);
tapestry.cleanConnect(&quot;btnUpload&quot;, &quot;onclick&quot;, &quot;event592121007&quot;);
        tapestry.event592121007=function(e){
            tapestry.event.stopEvent(e);
            
                
                    tapestry.form.submit(&quot;OnShelfForm&quot;, &quot;btnUpload&quot;, {&quot;async&quot;:true,&quot;json&quot;:false,&quot;url&quot;:&quot;/inspireapp/circulation/inventory/InventoryMaintain,OnShelfForm.sdirect?updateParts=InventoryDetails&amp;updateParts=nrResults&quot;});
                
                
            
            
        };
        tapestry.connect(&quot;btnUpload&quot;, &quot;onclick&quot;, &quot;event592121007&quot;);

tapestry.form.registerForm(&quot;Form&quot;);

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
closeDialogComponent(&quot; , &quot;'&quot; , &quot;TinreadMessageDialog&quot; , &quot;'&quot; , &quot;);

tapestry.form.focusField(&quot; , &quot;'&quot; , &quot;inputTxtField&quot; , &quot;'&quot; , &quot;);});
// -->



id(&quot;For_5&quot;)/td[6]&quot;) or . = concat(&quot;

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

dojo.require(&quot;tapestry.fx&quot;);
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
// -->


	
	
	







		
	
	
		
			
				
				Hi, catc (CU)
				
				最近登入:2024-03-13 13:30:02 ,成功
				
				
					
					
                       0 筆新訊息
                    
                
				
				繁體中文English繁體中文
				
English
繁體中文
						
						
				
				
					
					登 出
				
			
		
		
		
			
			
				神資圖書館(15trunk機)
			
			
		
	
	
	
	
	
		
		
		
			
			
			 查詢  編目  流通  採購   期刊控制   場地設備  說明  我的帳戶  學科服務  管理  統計  清單  Portal  臉辨報表  ERM 
			簡單查詢進階查詢FRBR查詢最近的查詢已儲存的記錄FRBR作品書目書目記錄維護註記刪除之書目記錄書目記錄全域修改權威記錄權威記錄維護註記刪除之權威記錄權威記錄全域修改條碼號館藏資料維護註記刪除之館藏資料維護出版者出版者資料維護內部移送作業書單維護FRBR作品辦證讀者記錄維護檢查讀者註記刪除之讀者記錄讀者證卡記錄維護離職離校借閱檯讀者流通檯還書箱讀者查詢交易紀錄違規讀者查詢讀者個人違規記錄指定參考書交易歷史查詢一般搜尋交易館藏預約順位調整題名預約順位調整批次修改到期日內部移送作業盤點盤點維護跨館預約/調閱跨館預約/調閱需求移轉-寄送項目移轉-收到項目盤點比對批次報廢選擇報廢執行報廢已報廢歷史查詢書箱作業條碼號空號清單(條碼機印製)訂購檢查薦購請購訂購明細訂單登收用卷期登收程序發票款項維護預算經費供應商標案內部移送作業催缺候選催缺項目催缺記錄期刊主檔訂購檢查薦購請購核可訂購明細訂單登收用卷期登收程序發票款項維護預算經費供應商裝訂裝訂主檔維護擬裝設定送裝確認裝訂登收標案內部移送作業催缺候選催缺項目催缺記錄流通記錄預約記錄借用記錄維護分類維護單項維護參數設定流通政策維護預約審核場地設備黑名單書籤訊息排程工作日誌學科館藏館藏資料類型設定學科館藏維護學科預約教師資料維護讀者評價選單學科預約維護學科預約查詢教師評語學科報表學科館藏冊數統計教師學科服務時數與使用人次統計學科服務讀者清單學科報表學科館藏冊數統計教師學科服務時數與使用人次統計學科服務讀者清單一般設定類型/使用者(館員)片語設定系統設定貨幣匯率館藏地範本設定--依館員帳號翻譯轉換 選取清單格式屬性條碼號規範顯示設定圖書館系統屬性推薦範本設定館員新增申請QR Code 設定編目書目類型設定MARC規範格式設定顯示設定停用字SDI維護同義字條碼號原則對應流通流通通知單範本流通通知單參數設定流通通知單發送紀錄日常作業排程一般每周開閉館設定閉館日期維護交易紀錄類型交易方式可適用讀者身份交易方式可適用館藏流通類別交易方式可適用使用類別交易紀錄-館藏狀態流通政策讀者證卡原則單位所系夾書單欄位設定讀者閱讀興趣發送紀錄期刊週期傳閱清單維護 轉入Z39屬性設定更新書封上傳書封索引編修索引重建功能維護公佈欄維護意見回覆維護主題維護特別書籍維護心得分享東區互惠審核東區互惠匯出系統代碼維護貨幣代碼作者號維護科特號維護OPACSDI維護讀者評論管理Help維護熱門關鍵字過濾館員Log記錄館員登入/出記錄館員資料變更記錄讀者資料變更記錄權限設定MARC權限設定功能權限設定角色設定系統狀態排程管理排程監控者排程日誌查詢統計檢索點/檢索語統計報表不同時段使用次數統計書目下載次數統計電子資源統計電子資源-館藏點閱次數統計電子資源-讀者點閱次數統計電子資源-單位系所點閱次數統計編目統計館藏量統計報表類號區間統計報表圖書增減統計報表編目/館別/大類交叉分析報表大類/冊數統計表大類/特藏交叉統計表特藏/冊數統計表新書特藏大類交叉統計圖書增減表／月份／大類大類語言種數冊數統計大類/條碼號類別冊數統計圖書冊數分類統計單圖書冊數金額分類統計非書資料數量金額統計資料類型範圍館藏統計表編目工作量統計(個人)編目工作量統計(各館)館藏狀態統計報表流通統計讀者借閱排行榜書展借閱清單流通人數統計(學制分頁 符合科系)讀者類型借閱暨歸還流通量統計報表流通人次冊數統計  利用概況統計(日報表)讀者借閱交叉分析各館特藏預約統計報表館內使用統計讀者身份借閱統計讀者個人及單位所系借閱排行榜館藏資料借閱統計單位系所借閱排行榜單位系所借閱統計熱門借閱排行榜熱門預約排行榜讀者辦証暨補發人次統計表借閱交叉分析統計資料類型流通統計館員流通量統計單位(個人)保管圖書(非書)數量明細館際互借代還統計-本館館藏館際互借代還統計-他館館藏指定參考書借閱量統計表各大類熱門借閱排行榜大專院校圖書館讀者身份借閱清單及統計報表館藏流通量統計跨館預約/調閱館藏借閱排行榜罰款收款人數/次數統計跨館預約/調閱使用量統計罰款收款統計特定館藏借閱統計採購統計書商採購統計年度經費採購統計特藏訂購方式金額統計書商到書率書商績效統計期刊統計期刊到刊率統計報表平均收刊日統計報表歷年期刊借閱統計書商到書率書商催缺次數書商累計訂購次數期刊訂費漲幅統計逾期違約金統計期刊經費訂購統計報表場地設備場地設備使用統計表報表模組範本維護編目清單書目標籤報表Tag856檢查報表新版書標維護次分類交叉分析登錄簿條碼產生清單登錄簿清單條碼號空號清單單位科系館藏設定清單館藏狀態預約清單特殊館藏清單館藏狀態清單報表流通清單罰款未收款報表預約待取清單借閱(逾期)清單還書清單新書清單讀者欠款清單借閱逾期清單罰款收款清單預約撤架清單長期借閱圖書清單離線流通清單預約額滿資料清單單位系所借閱逾期清單跨館還書清單離職離校清單跨館移送清單預約保留結束清單指定參考書借閱量明細表未外借館藏清單重複讀者比對報表借閱逾期通知單取書櫃上架清單預約圖書清單採購清單年度經費採買書籍運用率訂購資料明細表擬購資料明細表請購單詢價單訂購逾期清單謝函書的催缺報表薦購書刊複本比對報表採購參考清單財產增加清冊財產減少清冊已付款未到清單訂購明細期刊清單期刊合訂本登錄簿交贈期刊清單期刊架位清單裝訂歷史清單薦購書刊複本比對報表期刊登錄簿索贈清單期刊查驗清單已付款未到清單場地設備預約取消清單臉辨用戶使用分析表臉辨用戶與圖書分類關係表期間辦證統計_年報期間辦證統計_月報期間辦證明細期間辦證異動統計_年報期間辦證異動統計_月報辦證異動明細臉辨使用紀錄統計_年報臉辨使用紀錄統計_月報臉辨使用紀錄明細人數統計表人數統計明細表人次統計表人次統計明細表熱門時段統計表讀者進出排行榜期間辦證刪除明細系統參數設定IP區間管理延伸查詢設定系統設定檔資源瀏覽/後分類設定資源到期通知設定可使用身分類別設定 代碼管理所屬資料庫清單共用代碼檔共用代碼類別電子資源管理資源前端顯示設定Ezproxy設定檔維護電子資料庫/網路資源電子期刊/電子書電子資源批次匯入電子資源副檔匯入讀者回報資源連線異常 Meniuri.erm.9000報表點閱資源排行榜依學院別使用統計依單位別使用統計依月分資源使用統計依學院登入統計表依系所登入統計表依身分登入統計表登入清單依身分別使用統計表使用者使用資源清單 收藏資源排行榜偵測電子資源狀態設定偵測資源時間設定偵測電子資源查詢電子資源狀態電子資源狀況報表
				
	


		
			
				流通 > 盤點 > 盤點維護		
			

 
  建立範本    

     
   


	
			
				23 筆 •
				頁數 3 •
			
					
			
			
				
		       
			
			
			     
			
				
		    
			
				
				
		          
			      1
		         
				
			
				
				
		          
			      2
		         
				
			
				
					3  	
				
				
			
			
		     
			 
		     
			 	
			     
			 
			
	


	
     
	     
	      
	      
	       序號
	       功能圖示
	       盤點名稱
	       盤點狀態
	       備註
	       母條件搜尋語法
	      
	      
	      
	        21
	        
	          
	          
	          
	          
	        
	        20230424
	        啟動盤點
	        
	        條碼號 starts with &quot; , &quot;'&quot; , &quot;test&quot; , &quot;'&quot; , &quot; order by 條碼號
	      
	        22
	        
	          
	            
	          
	          
	          
	            
	          
	          
	        
	        2023
	        尚未啟動盤點
	        
	        條碼號 starts with &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; order by 條碼號
	      
	        23
	        
	          
	          
	          
	          
	        
	        測試盤點_20240313
	        啟動盤點
	        測試盤點_202403130145
	         (條碼號 starts with &quot; , &quot;'&quot; , &quot;0000000001&quot; , &quot;'&quot; , &quot;) and 條碼號 &lt;= &quot; , &quot;'&quot; , &quot;0000000018&quot; , &quot;'&quot; , &quot; order by 條碼號
	      
	     
   

	
			
				23 筆 •
				頁數 3 •
			
					
			
			
				
		       
			
			 
				
		    
			
				
				
		          
			      1
		         
				
			
				
				
		          
			      2
		         
				
			
				
					3  	
				
				
			
			
		     
			  
		     
			 	
			     
			 
				
		


   

   
   
   
     
     
      
       
         
          
           盤點母樣本數:
           2
           
          
          
           盤點上傳數:
           1
           清除資料
          
          
           前次作業:
           ACTIVATE
           
          
          
         
         條碼號
             &quot; , &quot;'&quot; , &quot;0000000001
         
		 
		   館藏狀態
		     
		 
		 
		   比對結果
		     不存在資料庫
		 
       
       
        
        
	         






	            
	            
	             刷入條碼號
	             
	             
	             輸入超過255字元
	            
	         
         
         








            
            

            
             上傳條碼號文字檔(.txt)

            
            
            
            
            上傳盤點檔案
            
            
            
            
         

        
       
      
     
     
	     
	      
	            使失效
	      
	     
	       檢查
	     
	     
	      執行
	     
	     
	     
	     取消
   
  
 

 

			
		
	




		
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

tapestry.form.registerForm(&quot;OnShelfForm&quot;);
tapestry.cleanConnect(&quot;btnUpload&quot;, &quot;onclick&quot;, &quot;event592121007&quot;);
        tapestry.event592121007=function(e){
            tapestry.event.stopEvent(e);
            
                
                    tapestry.form.submit(&quot;OnShelfForm&quot;, &quot;btnUpload&quot;, {&quot;async&quot;:true,&quot;json&quot;:false,&quot;url&quot;:&quot;/inspireapp/circulation/inventory/InventoryMaintain,OnShelfForm.sdirect?updateParts=InventoryDetails&amp;updateParts=nrResults&quot;});
                
                
            
            
        };
        tapestry.connect(&quot;btnUpload&quot;, &quot;onclick&quot;, &quot;event592121007&quot;);

tapestry.form.registerForm(&quot;Form&quot;);

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
closeDialogComponent(&quot; , &quot;'&quot; , &quot;TinreadMessageDialog&quot; , &quot;'&quot; , &quot;);

tapestry.form.focusField(&quot; , &quot;'&quot; , &quot;inputTxtField&quot; , &quot;'&quot; , &quot;);});
// -->



id(&quot;For_5&quot;)/td[6]&quot;))]</value>
      <webElementGuid>497b3ba7-25be-4398-8cfa-868b2d4ce191</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
