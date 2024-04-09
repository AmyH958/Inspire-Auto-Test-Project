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

WebUI.setText(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/input_(15trunk)_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/input_(15trunk)_member_pwd'), 
    'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/input_(15trunk)_Submit'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/a_'), '流通')

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/a_'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/a__1'), '指定參考書')

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/a__1'))

WebUI.verifyElementText(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/b_'), '查詢')

WebUI.verifyElementClickable(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/img__ImageSearch'))

WebUI.setText(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/input__field1'), '李')

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/h3_'))

WebUI.selectOptionByValue(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/select_andornot'), '1', 
    true)

WebUI.setText(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/input__field2'), 'A')

WebUI.selectOptionByValue(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/select_andornot_1'), '1', 
    true)

WebUI.selectOptionByValue(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/select_'), '8', true)

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/input_A100411899_field3'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/body_dojo.registerModulePath(tapestry, insp_118d36'))

WebUI.setText(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/input_A100411899_field3'), 'Java')

WebUI.selectOptionByValue(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/select_andornot_1_2'), 
    '1', true)

WebUI.setText(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/input_A100411899_field4'), '1')

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/input__formSubmitSearch'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/input__resetbutton'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/input__formSubmitSearch'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/td_'))

WebUI.setText(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/input__field1'), '鍾')

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/input__formSubmitSearch'))

WebUI.selectOptionByValue(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/select_'), '3', true)

WebUI.setText(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/input_A100411899_field3'), '0')

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/div_102050100500                           _29060f'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/input__formSubmitSearch'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/td__1'))

WebUI.selectOptionByValue(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/select__1'), '2', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/select_()()()()'), 'contains', 
    true)

WebUI.setText(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/input__field2'), '學')

WebUI.setText(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/input_A100411899_field3'), '0')

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/input__formSubmitSearch'))

WebUI.setText(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/input__field2'), '學')

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/input__formSubmitSearch'))

WebUI.selectOptionByValue(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/select__1_2'), '4', true)

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/input__DatePicker_2'))

WebUI.selectOptionByValue(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/select_()()()()_1'), '=', 
    true)

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/img__datePickerImg'))

WebUI.setText(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/input__DatePicker_2'), '2021/10/31')

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/div_()()()()                               _bf05d7'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/p_andornot()()()()'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/div_'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/input__DatePicker_2'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/div_reset_l w'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/input__formSubmitSearch'))

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/button_Clear'))

WebUI.takeFullPageScreenshot()

WebUI.enhancedClick(findTestObject('Object Repository/流通/指定參考書/根據查詢條件4Q欄位查詢/Page_(15trunk)/a__1_2'))

WebUI.closeBrowser()

