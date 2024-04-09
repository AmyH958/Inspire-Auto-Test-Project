import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('http://10.11.20.15/inspireapp/logon')

WebUI.setText(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/input_(15trunk)_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/input_(15trunk)_member_pwd'), 'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.enhancedClick(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/input_(15trunk)_Submit'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/a_'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/a__1'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/a__1_2'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/div_'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/a__1_2_3'), '書目')

WebUI.enhancedClick(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/a__1_2_3'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/label_Load file from'), 'Load file from')

WebUI.verifyElementText(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/select_BROWSER_UPLOADLOCAL_FILE_SYSTEM'), 'BROWSER_UPLOAD\nLOCAL_FILE_SYSTEM')

WebUI.verifyElementText(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/td_'), '檔案路徑:')

WebUI.verifyElementPresent(findTestObject('Page_(15trunk)/input__Upload'), 1)

WebUI.verifyElementText(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/label_Local file path'), 'Local file path')

WebUI.enhancedClick(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/input_Local file path_localFilePathField'))

WebUI.rightClick(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/input_Local file path_localFilePathField'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/div_ISO OPAC                               _450792'))

WebUI.verifyElementPresent(findTestObject('Page_(15trunk)/input_Local file path_localFilePathField'), 1)

WebUI.verifyElementText(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/a_ISO'), 'ISO 預覽')

WebUI.enhancedClick(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/a_ISO'))

WebUI.switchToWindowTitle('Details')

WebUI.verifyElementText(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/td_ (1)'), '譯碼:')

WebUI.verifyElementText(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/td__1'), '檔案路徑:')

WebUI.verifyElementText(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/a_ISO (1)'), 'ISO 預覽')

WebUI.verifyElementPresent(findTestObject('Page_Details/img_ISO_Any_8'), 1)

WebUI.enhancedClick(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/img_ISO_Any_8'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/p_ (1)'), '是否離開此頁面?')

WebUI.rightClick(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/a_ (1)'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/a_ (1)'), '是')

WebUI.switchToWindowIndex(0)

WebUI.enhancedClick(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/a_ (1)'))

WebUI.takeFullPageScreenshotAsCheckpoint('書目轉入查詢')

WebUI.switchToWindowTitle('神資圖書館(15trunk機)')

WebUI.enhancedClick(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/a__1_2_3_4'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/a__1_2_3_4_5'))

WebUI.rightClick(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/a__1_2_3_4_5_6'))

WebUI.executeJavaScript('', [])

WebUI.verifyElementText(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/a__1_2_3_4_5_6_7'), '權威記錄')

WebUI.enhancedClick(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/a__1_2_3_4_5'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/a__1_2_3_4_5_6_7_8'), '薦購')

WebUI.enhancedClick(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/a__1_2_3_4_5_6_7_8_9'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/a__1_2_3_4_5_6_7_8_9_10'), '讀者')

WebUI.enhancedClick(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/a_Loans'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/a_Loans_1'), 'Loans')

WebUI.enhancedClick(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/a__1_2_3_4_5_6_7_8_9_10_11'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/a__1_2_3_4_5_6_7_8_9_10_11_12'), '違規記錄')

WebUI.enhancedClick(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/a__1_2_3_4_5_6_7_8_9_10_11_12_13'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/a__1_2_3_4_5_6_7_8_9_10_11_12_13_14'), '供應商')

WebUI.enhancedClick(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/a__1_2_3_4_5_6_7_8_9_10_11_12_13_14_15'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/a__1_2_3_4_5_6_7_8_9_10_11_12_13_14_15_16'), '訂購明細')

WebUI.verifyElementText(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/a__1_2_3_4_5_6_7_8_9_10_11_12_13_14_15_16_17'), 
    '範例檔下載')

WebUI.enhancedClick(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/a__1_2_3_4_5_6_7_8_9_10_11_12_13_14_15_16_17'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/td'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/p_'), '說明')

WebUI.verifyElementText(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/p_1'), '1.此功能僅更新採購及期刊模組訂購明細之部份欄位資料，並以覆蓋已有訂購明細資料為主，故請正確勾選欲更新的「其他欄位」，若指定更新的欄位內容為空時，將清空該欄位資料。')

WebUI.verifyElementText(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/p_2'), '2.若要修改之訂購明細資料內容皆一致，建議改至採購或期刊模組之訂購明細全域修正功能處理之。')

WebUI.enhancedClick(findTestObject('Object Repository/管理/維護/轉入/轉入查詢/a__1_2_3_4_5_6_7_8_9_10_11_12_13_14_15_16_17_18'))

WebUI.closeBrowser()

