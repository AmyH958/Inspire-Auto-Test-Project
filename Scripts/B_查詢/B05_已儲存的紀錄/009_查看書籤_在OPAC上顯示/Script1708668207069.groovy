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

WebUI.maximizeWindow()

WebUI.setText(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤在OPAC上顯示/Page_(15trunk)/input_(15trunk)_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤在OPAC上顯示/Page_(15trunk)/input_(15trunk)_member_pwd'), 
    'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.click(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤在OPAC上顯示/Page_(15trunk)/input_(15trunk)_Submit'))

WebUI.verifyElementText(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤在OPAC上顯示/Page_(15trunk)/a_'), '查詢')

WebUI.click(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤在OPAC上顯示/Page_(15trunk)/a_'))

WebUI.verifyElementText(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤在OPAC上顯示/Page_(15trunk)/a__1'), '已儲存的記錄')

WebUI.enhancedClick(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤在OPAC上顯示/Page_(15trunk)/a__1'))

WebUI.verifyElementClickable(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤在OPAC上顯示/Page_(15trunk)/input__selectat'))

WebUI.enhancedClick(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤在OPAC上顯示/Page_(15trunk)/input__selectat'))

WebUI.enhancedClick(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤在OPAC上顯示/Page_(15trunk)/a_OPAC'))

WebUI.scrollToElement(findTestObject('查詢/以儲存的紀錄/查看書籤在OPAC上顯示/Page_(15trunk)/div_'), 2)

WebUI.verifyElementText(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤在OPAC上顯示/Page_(15trunk)/div_'), '提示註記')

WebUI.verifyElementText(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤在OPAC上顯示/Page_(15trunk)/div_OPAC'), '選擇的紀錄是否於OPAC上顯示')

WebUI.verifyElementText(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤在OPAC上顯示/Page_(15trunk)/span_'), '是')

WebUI.enhancedClick(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤在OPAC上顯示/Page_(15trunk)/span_'))

WebUI.verifyElementText(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤在OPAC上顯示/Page_(15trunk)/div__1'), '您的資料已進入排程處理，請至"排程監控者"查看進度，待處理完畢時即可重新查詢。')

WebUI.verifyElementText(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤在OPAC上顯示/Page_(15trunk)/span__1'), '確定')

WebUI.enhancedClick(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤在OPAC上顯示/Page_(15trunk)/span__1'))

WebUI.enhancedClick(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤在OPAC上顯示/Page_(15trunk)/a__1_2'))

WebUI.closeBrowser()

