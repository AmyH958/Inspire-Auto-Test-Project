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

WebUI.setText(findTestObject('Object Repository/查詢/進階查詢/根據查詢條件4Q欄位查詢/Page_(15trunk)/input_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/查詢/進階查詢/根據查詢條件4Q欄位查詢/Page_(15trunk)/input_member_pwd'), 'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.click(findTestObject('Object Repository/查詢/進階查詢/根據查詢條件4Q欄位查詢/Page_(15trunk)/input_Submit'))

WebUI.verifyElementText(findTestObject('Object Repository/查詢/進階查詢/根據查詢條件4Q欄位查詢/Page_(15trunk)/a_'), '查詢')

WebUI.click(findTestObject('Object Repository/查詢/進階查詢/根據查詢條件4Q欄位查詢/Page_(15trunk)/a_'))

WebUI.verifyElementText(findTestObject('Object Repository/查詢/進階查詢/根據查詢條件4Q欄位查詢/Page_(15trunk)/a__1'), '進階查詢')

WebUI.click(findTestObject('Object Repository/查詢/進階查詢/根據查詢條件4Q欄位查詢/Page_(15trunk)/a__1'))

WebUI.selectOptionByValue(findTestObject('Object Repository/查詢/進階查詢/根據查詢條件4Q欄位查詢/Page_(15trunk)/select_ISBNISSN'), '1', 
    true)

WebUI.selectOptionByValue(findTestObject('Object Repository/查詢/進階查詢/根據查詢條件4Q欄位查詢/Page_(15trunk)/select_()()()()'), 'contains', 
    true)

WebUI.selectOptionByValue(findTestObject('Object Repository/查詢/進階查詢/根據查詢條件4Q欄位查詢/Page_(15trunk)/select_()()()()'), 'starts with', 
    true)

WebUI.click(findTestObject('Object Repository/查詢/進階查詢/根據查詢條件4Q欄位查詢/Page_(15trunk)/input_field1'))

WebUI.verifyElementPresent(findTestObject('Object Repository/查詢/進階查詢/根據查詢條件4Q欄位查詢/Page_(15trunk)/input_field1'), 0)

WebUI.setText(findTestObject('Object Repository/查詢/進階查詢/根據查詢條件4Q欄位查詢/Page_(15trunk)/input_field1'), 'Java')

WebUI.setText(findTestObject('Object Repository/查詢/進階查詢/根據查詢條件4Q欄位查詢/Page_(15trunk)/input_field2'), '1')

WebUI.selectOptionByValue(findTestObject('Object Repository/查詢/進階查詢/根據查詢條件4Q欄位查詢/Page_(15trunk)/select_()()()()_1'), 'starts with', 
    true)

WebUI.selectOptionByValue(findTestObject('Object Repository/查詢/進階查詢/根據查詢條件4Q欄位查詢/Page_(15trunk)/select_()()()()_1'), 'contains', 
    true)

WebUI.setText(findTestObject('Object Repository/查詢/進階查詢/根據查詢條件4Q欄位查詢/Page_(15trunk)/input_field3'), '1')

WebUI.selectOptionByValue(findTestObject('Object Repository/查詢/進階查詢/根據查詢條件4Q欄位查詢/Page_(15trunk)/select_ISBNISSN_1_2'), '2', 
    true)

WebUI.setText(findTestObject('Object Repository/查詢/進階查詢/根據查詢條件4Q欄位查詢/Page_(15trunk)/input_field4'), '李')

WebUI.selectOptionByValue(findTestObject('Object Repository/查詢/進階查詢/根據查詢條件4Q欄位查詢/Page_(15trunk)/select_()()()()_1_2'), 'starts with', 
    true)

WebUI.selectOptionByValue(findTestObject('Object Repository/查詢/進階查詢/根據查詢條件4Q欄位查詢/Page_(15trunk)/select_()()()()_1_2'), 'contains', 
    true)

WebUI.verifyElementText(findTestObject('Object Repository/查詢/進階查詢/根據查詢條件4Q欄位查詢/Page_(15trunk)/input_formSubmitSearch'), 
    '')

WebUI.click(findTestObject('Object Repository/查詢/進階查詢/根據查詢條件4Q欄位查詢/Page_(15trunk)/input_formSubmitSearch'))

WebUI.setText(findTestObject('Object Repository/查詢/進階查詢/根據查詢條件4Q欄位查詢/Page_(15trunk)/input_field3'), '')

WebUI.setText(findTestObject('Object Repository/查詢/進階查詢/根據查詢條件4Q欄位查詢/Page_(15trunk)/input_field2'), '')

WebUI.click(findTestObject('Object Repository/查詢/進階查詢/根據查詢條件4Q欄位查詢/Page_(15trunk)/input_formSubmitSearch'))

WebUI.takeFullPageScreenshot()

WebUI.closeBrowser()

