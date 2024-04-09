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

WebUI.setText(findTestObject('Object Repository/查詢/進階查詢/依限制條件查詢/Page_(15trunk)/input_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/查詢/進階查詢/依限制條件查詢/Page_(15trunk)/input_member_pwd'), 'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.click(findTestObject('Object Repository/查詢/進階查詢/依限制條件查詢/Page_(15trunk)/input_Submit'))

WebUI.verifyElementText(findTestObject('Object Repository/查詢/進階查詢/依限制條件查詢/Page_(15trunk)/a_'), '限制條件')

WebUI.click(findTestObject('Object Repository/查詢/進階查詢/依限制條件查詢/Page_(15trunk)/a_'))

WebUI.verifyElementText(findTestObject('Object Repository/查詢/進階查詢/依限制條件查詢/Page_(15trunk)/td_'), '書目類型:')

WebUI.selectOptionByValue(findTestObject('Object Repository/查詢/進階查詢/依限制條件查詢/Page_(15trunk)/select_CD- CDBOOK- BOOKEP- EPBD- BDP- POthe_48a6d2'), 
    '2', true)

WebUI.verifyElementText(findTestObject('Object Repository/查詢/進階查詢/依限制條件查詢/Page_(15trunk)/td__1'), '館藏地:')

WebUI.verifyElementVisible(findTestObject('Object Repository/查詢/進階查詢/依限制條件查詢/Page_(15trunk)/img_open'))

WebUI.click(findTestObject('Object Repository/查詢/進階查詢/依限制條件查詢/Page_(15trunk)/img_open'))

WebUI.verifyElementClickable(findTestObject('Object Repository/查詢/進階查詢/依限制條件查詢/Page_(15trunk)/label'))

WebUI.click(findTestObject('Object Repository/查詢/進階查詢/依限制條件查詢/Page_(15trunk)/label'))

WebUI.verifyElementText(findTestObject('Object Repository/查詢/進階查詢/依限制條件查詢/Page_(15trunk)/button_'), '確定')

WebUI.click(findTestObject('Object Repository/查詢/進階查詢/依限制條件查詢/Page_(15trunk)/button_'))

WebUI.verifyElementText(findTestObject('Object Repository/查詢/進階查詢/依限制條件查詢/Page_(15trunk)/td__1_2'), '館藏狀態:')

WebUI.selectOptionByValue(findTestObject('Object Repository/查詢/進階查詢/依限制條件查詢/Page_(15trunk)/select_PickListValues.15004108CWEN'), 
    '1', true)

WebUI.verifyElementText(findTestObject('Object Repository/查詢/進階查詢/依限制條件查詢/Page_(15trunk)/input_formSubmitSearch'), '')

WebUI.scrollToElement(findTestObject('查詢/進階查詢/依限制條件查詢/Page_(15trunk)/input_formSubmitSearch'), 2)

WebUI.click(findTestObject('Object Repository/查詢/進階查詢/依限制條件查詢/Page_(15trunk)/a_'))

WebUI.verifyElementPresent(findTestObject('Object Repository/查詢/進階查詢/依限制條件查詢/Page_(15trunk)/div_CDATAfunction SelAll()var numjQuery(inp_07ce73'), 
    2)

WebUI.takeFullPageScreenshot()

WebUI.closeBrowser()

