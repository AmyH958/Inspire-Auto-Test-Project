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

WebUI.setText(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤點選明細/Page_(15trunk)/input_(15trunk)_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤點選明細/Page_(15trunk)/input_(15trunk)_member_pwd'), 
    'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.click(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤點選明細/Page_(15trunk)/input_(15trunk)_Submit'))

WebUI.verifyElementText(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤點選明細/Page_(15trunk)/a_'), '查詢')

WebUI.click(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤點選明細/Page_(15trunk)/a_'))

WebUI.verifyElementText(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤點選明細/Page_(15trunk)/a__1'), '已儲存的記錄')

WebUI.click(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤點選明細/Page_(15trunk)/a__1'))

WebUI.verifyElementText(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤點選明細/Page_(15trunk)/td_'), '編輯')

WebUI.click(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤點選明細/Page_(15trunk)/img__Any_0_0'))

WebUI.switchToWindowTitle('Details')

WebUI.click(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤點選明細/Page_Details/a_'))

WebUI.verifyElementText(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤點選明細/Page_Details/a__1'), '館藏')

WebUI.click(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤點選明細/Page_Details/a__1_2'))

WebUI.verifyElementText(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤點選明細/Page_Details/a__1_2_3'), '多媒體')

WebUI.click(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤點選明細/Page_Details/a__1_2_3_4'))

WebUI.verifyElementText(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤點選明細/Page_Details/input__Submit'), '')

WebUI.click(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤點選明細/Page_Details/input__Submit'))

WebUI.verifyElementText(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤點選明細/Page_Details/div_'), '存檔成功')

WebUI.verifyElementText(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤點選明細/Page_Details/span_'), '確定')

WebUI.click(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤點選明細/Page_Details/span_'))

WebUI.verifyElementPresent(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤點選明細/Page_Details/img__Any_7'), 0)

WebUI.click(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤點選明細/Page_Details/img__Any_7'))

WebUI.verifyElementText(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤點選明細/Page_Details/div__1'), '提示')

WebUI.verifyElementText(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤點選明細/Page_Details/p_'), '是否離開此頁面?')

WebUI.click(findTestObject('Object Repository/查詢/以儲存的紀錄/查看書籤點選明細/Page_Details/a__1_2_3_4_5'))

WebUI.closeBrowser()

