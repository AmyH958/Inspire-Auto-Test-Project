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

WebUI.setText(findTestObject('Object Repository/查詢/進階查詢/正確新增一本書籍/Page_(15trunk)/input_(15trunk)_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/查詢/進階查詢/正確新增一本書籍/Page_(15trunk)/input_(15trunk)_member_pwd'), 'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.click(findTestObject('Object Repository/查詢/進階查詢/正確新增一本書籍/Page_(15trunk)/input_(15trunk)_Submit'))

WebUI.verifyElementText(findTestObject('Object Repository/查詢/進階查詢/正確新增一本書籍/Page_(15trunk)/a_'), '查詢')

WebUI.click(findTestObject('Object Repository/查詢/進階查詢/正確新增一本書籍/Page_(15trunk)/a_'))

WebUI.verifyElementText(findTestObject('Object Repository/查詢/進階查詢/正確新增一本書籍/Page_(15trunk)/a__1'), '進階查詢')

WebUI.click(findTestObject('Object Repository/查詢/進階查詢/正確新增一本書籍/Page_(15trunk)/a__1'))

WebUI.click(findTestObject('Object Repository/查詢/進階查詢/正確新增一本書籍/Page_(15trunk)/a__1_2'))

WebUI.switchToWindowTitle('Details')

WebUI.verifyElementText(findTestObject('Object Repository/查詢/進階查詢/正確新增一本書籍/Page_Details/input__Submit'), '')

WebUI.click(findTestObject('Object Repository/查詢/進階查詢/正確新增一本書籍/Page_Details/input__Submit'))

WebUI.verifyElementText(findTestObject('Object Repository/查詢/進階查詢/正確新增一本書籍/Page_Details/span_'), '定長代碼欄')

WebUI.setText(findTestObject('Object Repository/查詢/進階查詢/正確新增一本書籍/Page_Details/input_a -             (NR)_subfieldId10_2'), 
    CreateNewBook)

WebUI.click(findTestObject('Object Repository/查詢/進階查詢/正確新增一本書籍/Page_Details/img_Work_dojoComboBox auto_complete'))

WebUI.verifyElementText(findTestObject('Object Repository/查詢/進階查詢/正確新增一本書籍/Page_Details/a_'), '修改/存檔')

WebUI.click(findTestObject('Object Repository/查詢/進階查詢/正確新增一本書籍/Page_Details/a_'))

WebUI.verifyElementText(findTestObject('Object Repository/查詢/進階查詢/正確新增一本書籍/Page_Details/div_'), '存檔成功')

WebUI.verifyElementText(findTestObject('Object Repository/查詢/進階查詢/正確新增一本書籍/Page_Details/span__1'), '確定')

WebUI.click(findTestObject('Object Repository/查詢/進階查詢/正確新增一本書籍/Page_Details/span__1'))

WebUI.verifyElementClickable(findTestObject('Object Repository/查詢/進階查詢/正確新增一本書籍/Page_Details/img__Any_237'))

WebUI.click(findTestObject('Object Repository/查詢/進階查詢/正確新增一本書籍/Page_Details/img__Any_237'))

WebUI.verifyElementText(findTestObject('Object Repository/查詢/進階查詢/正確新增一本書籍/Page_Details/p_'), '是否離開此頁面?')

WebUI.enableSmartWait()

WebUI.click(findTestObject('Object Repository/查詢/進階查詢/正確新增一本書籍/Page_Details/a__1'))

WebUI.switchToDefaultContent()

WebUI.verifyElementText(findTestObject('Object Repository/查詢/進階查詢/正確新增一本書籍/Page_(15trunk)/a__1_2_3'), '登 出')

WebUI.click(findTestObject('Object Repository/查詢/進階查詢/正確新增一本書籍/Page_(15trunk)/a__1_2_3'))

WebUI.closeBrowser()

